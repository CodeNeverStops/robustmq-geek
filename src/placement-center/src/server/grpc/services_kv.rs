// Copyright 2023 RobustMQ Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! gRPC KV 服务实现
//!
//! 该文件实现了 Placement Center 的 gRPC KV 服务，提供：
//! 1. KV 存储接口（set、get、delete、exists）
//! 2. 主从节点自动转发
//! 3. Raft 状态机集成
//! 4. 错误处理与响应
//!
//! 实现原理：
//! - 使用 tonic 框架实现 gRPC 服务
//! - 结合 Raft 状态机实现分布式一致性
//! - 支持主从节点自动路由
//! - 提供详细的错误处理机制

use std::sync::{Arc, RwLock};

use crate::{
    openraft::typeconfig::TypeConfig,
    raft::{
        apply::{RaftMachineApply, StorageData, StorageDataType},
        metadata::RaftGroupMetadata,
    },
    storage::{kv::KvStorage, rocksdb::RocksDBEngine},
};
use clients::{
    placement::kv::call::{placement_delete, placement_set},
    poll::ClientPool,
};
use common_base::errors::RobustMQError;
use openraft::Raft;
use prost::Message;
use protocol::kv::{
    kv_service_server::KvService, CommonReply, DeleteRequest, ExistsReply, ExistsRequest, GetReply,
    GetRequest, SetRequest,
};
use tonic::{Request, Response, Status};

pub struct GrpcKvServices {
    client_poll: Arc<ClientPool>,
    placement_center_storage: Arc<RaftMachineApply>,
    rocksdb_engine_handler: Arc<RocksDBEngine>,
    placement_cluster: Arc<RwLock<RaftGroupMetadata>>,
}

impl GrpcKvServices {
    pub fn new(
        client_poll: Arc<ClientPool>,
        placement_center_storage: Arc<RaftMachineApply>,
        rocksdb_engine_handler: Arc<RocksDBEngine>,
        placement_cluster: Arc<RwLock<RaftGroupMetadata>>,
    ) -> Self {
        return GrpcKvServices {
            client_poll,
            placement_center_storage,
            rocksdb_engine_handler,
            placement_cluster,
        };
    }

    pub fn is_leader(&self) -> bool {
        return self.placement_cluster.read().unwrap().is_leader();
    }

    pub fn leader_addr(&self) -> String {
        return self.placement_cluster.read().unwrap().leader_addr();
    }
}

#[tonic::async_trait]
impl KvService for GrpcKvServices {
    async fn set(&self, request: Request<SetRequest>) -> Result<Response<CommonReply>, Status> {
        let req = request.into_inner();

        if req.key.is_empty() || req.value.is_empty() {
            return Err(Status::cancelled(
                RobustMQError::ParameterCannotBeNull("key or value".to_string()).to_string(),
            ));
        }

        if !self.is_leader() {
            let leader_addr = self.leader_addr();
            match placement_set(self.client_poll.clone(), vec![leader_addr], req).await {
                Ok(reply) => {
                    return Ok(Response::new(reply));
                }
                Err(e) => {
                    return Err(Status::cancelled(e.to_string()));
                }
            }
        }

        // Raft state machine is used to store Node data
        let data = StorageData::new(StorageDataType::KvSet, SetRequest::encode_to_vec(&req));
        match self
            .placement_center_storage
            .apply_propose_message(data, "set".to_string())
            .await
        {
            Ok(_) => return Ok(Response::new(CommonReply::default())),
            Err(e) => {
                return Err(Status::cancelled(e.to_string()));
            }
        }
    }

    async fn delete(
        &self,
        request: Request<DeleteRequest>,
    ) -> Result<Response<CommonReply>, Status> {
        let req = request.into_inner();

        if req.key.is_empty() {
            return Err(Status::cancelled(
                RobustMQError::ParameterCannotBeNull("key".to_string()).to_string(),
            ));
        }

        if !self.is_leader() {
            let leader_addr = self.leader_addr();
            match placement_delete(self.client_poll.clone(), vec![leader_addr], req).await {
                Ok(reply) => {
                    return Ok(Response::new(reply));
                }
                Err(e) => {
                    return Err(Status::cancelled(e.to_string()));
                }
            }
        }

        // Raft state machine is used to store Node data
        let data = StorageData::new(
            StorageDataType::KvDelete,
            DeleteRequest::encode_to_vec(&req),
        );
        match self
            .placement_center_storage
            .apply_propose_message(data, "delete".to_string())
            .await
        {
            Ok(_) => return Ok(Response::new(CommonReply::default())),
            Err(e) => {
                return Err(Status::cancelled(e.to_string()));
            }
        }
    }

    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetReply>, Status> {
        let req = request.into_inner();

        if req.key.is_empty() {
            return Err(Status::cancelled(
                RobustMQError::ParameterCannotBeNull("key".to_string()).to_string(),
            ));
        }

        let kv_storage = KvStorage::new(self.rocksdb_engine_handler.clone());
        let mut reply = GetReply::default();
        match kv_storage.get(req.key) {
            Ok(Some(data)) => {
                reply.value = data;
                return Ok(Response::new(reply));
            }
            Ok(None) => {}
            Err(e) => return Err(Status::cancelled(e.to_string())),
        }

        return Ok(Response::new(reply));
    }

    async fn exists(
        &self,
        request: Request<ExistsRequest>,
    ) -> Result<Response<ExistsReply>, Status> {
        let req = request.into_inner();

        if req.key.is_empty() {
            return Err(Status::cancelled(
                RobustMQError::ParameterCannotBeNull("key".to_string()).to_string(),
            ));
        }

        let kv_storage = KvStorage::new(self.rocksdb_engine_handler.clone());
        match kv_storage.exists(req.key) {
            Ok(flag) => {
                return Ok(Response::new(ExistsReply { flag }));
            }
            Err(e) => {
                return Err(Status::cancelled(e.to_string()));
            }
        }
    }
}
