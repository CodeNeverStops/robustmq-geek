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

//! Raft 节点间消息转发
//!
//! 该文件实现了 Raft 节点间的消息转发机制，包括：
//! 1. PeerMessage 结构定义
//! 2. 节点消息异步转发
//! 3. gRPC 客户端调用
//! 4. 错误与重试处理
//!
//! 实现原理：
//! - 使用 tokio mpsc channel 实现异步消息队列
//! - 通过 gRPC 客户端发送 Raft 消息
//! - 支持日志输出与错误处理

use log::{debug, error, info};
use protocol::placement::{
    placement_center_service_client::PlacementCenterServiceClient, SendRaftMessageRequest,
};
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub struct PeerMessage {
    pub to: String,
    pub data: Vec<u8>,
}

pub struct PeersManager {
    peer_message_recv: mpsc::Receiver<PeerMessage>,
}

impl PeersManager {
    pub fn new(peer_message_recv: mpsc::Receiver<PeerMessage>) -> PeersManager {
        let pm = PeersManager { peer_message_recv };
        return pm;
    }

    pub async fn start(&mut self) {
        info!(
            "{}",
            "Starts the thread that sends Raft messages to other nodes"
        );
        loop {
            if let Some(data) = self.peer_message_recv.recv().await {
                let addr = data.to;
                let request: SendRaftMessageRequest = SendRaftMessageRequest { message: data.data };
                let mut client = PlacementCenterServiceClient::connect(format!("http://{}", addr))
                    .await
                    .unwrap();

                match client.send_raft_message(request).await {
                    Ok(_) => debug!("Send Raft message to node {} Successful.", addr),
                    Err(e) => error!(
                        "Failed to send data to {}, error message: {}",
                        addr,
                        e.to_string()
                    ),
                }
            }
        }
    }
}
