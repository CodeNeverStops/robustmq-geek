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

//! Raft 状态机消息与应用
//!
//! 该文件定义了 Raft 状态机的消息类型、数据结构及其应用逻辑，包括：
//! 1. Raft 消息类型定义
//! 2. 状态机数据结构
//! 3. 消息发送与响应机制
//! 4. 客户端请求与响应处理
//!
//! 实现原理：
//! - 使用 tokio channel 进行异步消息传递
//! - 支持多种消息类型（配置变更、普通消息、主节点转移、客户端提案）
use bincode::serialize;
use common_base::errors::RobustMQError;
use raft::eraftpb::ConfChange;
use raft::eraftpb::Message as raftPreludeMessage;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::time::Duration;
use tokio::sync::oneshot;
use tokio::sync::oneshot::Receiver;
use tokio::sync::oneshot::Sender;
use tokio::time::timeout;

pub enum RaftResponseMesage {
    Success,
    Fail,
}
pub enum RaftMessage {
    ConfChange {
        change: ConfChange,
        chan: Sender<RaftResponseMesage>,
    },

    // Received a message from another node
    Raft {
        message: raftPreludeMessage,
        chan: Sender<RaftResponseMesage>,
    },

    TransferLeader {
        node_id: u64,
        chan: Sender<RaftResponseMesage>,
    },

    // The data sent by the client is received. Procedure
    Propose {
        data: Vec<u8>,
        chan: Sender<RaftResponseMesage>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub enum StorageDataType {
    // kv
    KvSet,
    KvDelete,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StorageData {
    pub data_type: StorageDataType,
    pub value: Vec<u8>,
}

impl StorageData {
    pub fn new(data_type: StorageDataType, value: Vec<u8>) -> StorageData {
        return StorageData {
            data_type,
            value: value,
        };
    }
}

impl fmt::Display for StorageData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.data_type, self.value)
    }
}
pub struct RaftMachineApply {
    raft_status_machine_sender: tokio::sync::mpsc::Sender<RaftMessage>,
}

impl RaftMachineApply {
    pub fn new(raft_sender: tokio::sync::mpsc::Sender<RaftMessage>) -> Self {
        return RaftMachineApply {
            raft_status_machine_sender: raft_sender,
        };
    }

    pub async fn transfer_leader(&self, node_id: u64) -> Result<(), RobustMQError> {
        let (sx, rx) = oneshot::channel::<RaftResponseMesage>();
        return Ok(self
            .apply_raft_status_machine_message(
                RaftMessage::TransferLeader {
                    node_id: node_id,
                    chan: sx,
                },
                "transfer_leader".to_string(),
                rx,
            )
            .await?);
    }

    pub async fn apply_propose_message(
        &self,
        data: StorageData,
        action: String,
    ) -> Result<(), RobustMQError> {
        let (sx, rx) = oneshot::channel::<RaftResponseMesage>();
        return Ok(self
            .apply_raft_status_machine_message(
                RaftMessage::Propose {
                    data: serialize(&data).unwrap(),
                    chan: sx,
                },
                action,
                rx,
            )
            .await?);
    }

    pub async fn apply_raft_message(
        &self,
        message: raftPreludeMessage,
        action: String,
    ) -> Result<(), RobustMQError> {
        let (sx, rx) = oneshot::channel::<RaftResponseMesage>();
        return Ok(self
            .apply_raft_status_machine_message(
                RaftMessage::Raft {
                    message: message,
                    chan: sx,
                },
                action,
                rx,
            )
            .await?);
    }

    pub async fn apply_conf_raft_message(
        &self,
        change: ConfChange,
        action: String,
    ) -> Result<(), RobustMQError> {
        let (sx, rx) = oneshot::channel::<RaftResponseMesage>();
        return Ok(self
            .apply_raft_status_machine_message(
                RaftMessage::ConfChange { change, chan: sx },
                action,
                rx,
            )
            .await?);
    }

    async fn apply_raft_status_machine_message(
        &self,
        message: RaftMessage,
        action: String,
        rx: Receiver<RaftResponseMesage>,
    ) -> Result<(), RobustMQError> {
        let _ = self.raft_status_machine_sender.send(message).await;
        if !self.wait_recv_chan_resp(rx).await {
            return Err(RobustMQError::RaftLogCommitTimeout(action));
        }
        return Ok(());
    }

    async fn wait_recv_chan_resp(&self, rx: Receiver<RaftResponseMesage>) -> bool {
        let res = timeout(Duration::from_secs(30), async {
            match rx.await {
                Ok(val) => {
                    return val;
                }
                Err(_) => {
                    return RaftResponseMesage::Fail;
                }
            }
        });
        match res.await {
            Ok(_) => return true,
            Err(_) => {
                return false;
            }
        }
    }
}
