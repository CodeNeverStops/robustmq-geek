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

//! gRPC KV 新版服务实现
//!
//! 该文件实现了 Placement Center 的新版 gRPC KV 服务，提供：
//! 1. KV 存储接口（set、get、delete、exists）
//! 2. OpenRaft 状态机集成
//! 3. 主从节点自动路由
//! 4. 错误处理与响应
//!
//! 实现原理：
//! - 使用 tonic 框架实现 gRPC 服务
//! - 结合 OpenRaft 状态机实现分布式一致性
//! - 支持主从节点自动路由
//! - 提供详细的错误处理机制

use std::sync::{Arc, RwLock};

use crate::{
    openraft::{route::AppRequestData, typeconfig::TypeConfig},
    raft::{apply::RaftMachineApply, metadata::RaftGroupMetadata},
    storage::{kv::KvStorage, rocksdb::RocksDBEngine},
};
use bincode::serialize;
use clients::poll::ClientPool;
use common_base::errors::RobustMQError;
use openraft::Raft;
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
    raft_node: Raft<TypeConfig>,
}

impl GrpcKvServices {
    pub fn new(
        client_poll: Arc<ClientPool>,
        placement_center_storage: Arc<RaftMachineApply>,
        rocksdb_engine_handler: Arc<RocksDBEngine>,
        placement_cluster: Arc<RwLock<RaftGroupMetadata>>,
        raft_node: Raft<TypeConfig>,
    ) -> Self {
        return GrpcKvServices {
            client_poll,
            placement_center_storage,
            rocksdb_engine_handler,
            placement_cluster,
            raft_node,
        };
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

        let data = AppRequestData::Set {
            key: "k1".to_string(),
            value: "v1".to_string(),
        };

        match self.raft_node.client_write(data).await {
            Ok(data) => {
                return Ok(Response::new(CommonReply::default()));
            }
            Err(e) => {
                return Err(Status::cancelled(e.to_string()));
            }
        };
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

        let data = AppRequestData::Delete {
            key: "k1".to_string(),
        };

        match self.raft_node.client_write(data).await {
            Ok(data) => {
                return Ok(Response::new(CommonReply::default()));
            }
            Err(e) => {
                return Err(Status::cancelled(e.to_string()));
            }
        };
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
