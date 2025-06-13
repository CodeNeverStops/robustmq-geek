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

//! HTTP OpenRaft 相关接口实现
//!
//! 该文件实现了 Placement Center 的 OpenRaft 相关 HTTP 接口，提供：
//! 1. 集群成员管理（添加学习者、变更成员、初始化）
//! 2. 集群监控指标查询
//! 3. KV 数据写入与读取
//!
//! 实现原理：
//! - 使用 axum 框架实现 HTTP 路由
//! - 结合 OpenRaft 状态机实现分布式一致性
//! - 支持异步处理与统一响应格式

use std::collections::{BTreeMap, BTreeSet};

use axum::extract::State;
use common_base::http_response::{error_response, success_response};
use openraft::{error::Infallible, RaftMetrics};

use crate::openraft::{raft_node::Node, route::AppRequestData, typeconfig::TypeConfig};

use super::server::HttpServerState;

pub async fn add_leadrner(State(state): State<HttpServerState>) -> String {
    let node_id = 3;
    let node = Node {
        rpc_addr: "127.0.0.0:7654".to_string(),
        node_id: 2,
    };
    match state.raft_node.add_learner(node_id, node, true).await {
        Ok(data) => {
            return success_response(data);
        }
        Err(e) => {
            return error_response(e.to_string());
        }
    }
}

pub async fn change_membership(State(state): State<HttpServerState>) -> String {
    let mut body = BTreeSet::new();
    body.insert(3);
    match state.raft_node.change_membership(body, true).await {
        Ok(data) => {
            return success_response(data);
        }
        Err(e) => {
            return error_response(e.to_string());
        }
    }
}

pub async fn init(State(state): State<HttpServerState>) -> String {
    let node_id = 3;
    let node = Node {
        rpc_addr: "127.0.0.0:7654".to_string(),
        node_id: 2,
    };

    let mut nodes = BTreeMap::new();
    nodes.insert(node_id, node);

    match state.raft_node.initialize(nodes).await {
        Ok(data) => {
            return success_response(data);
        }
        Err(e) => {
            return error_response(e.to_string());
        }
    }
}

pub async fn metrics(State(state): State<HttpServerState>) -> String {
    let metrics = state.raft_node.metrics().borrow().clone();
    let res: Result<RaftMetrics<TypeConfig>, Infallible> = Ok(metrics);
    return success_response(res);
}

pub async fn set(State(state): State<HttpServerState>) -> String {
    let data = AppRequestData::Set {
        key: "k1".to_string(),
        value: "v1".to_string(),
    };
    match state.raft_node.client_write(data).await {
        Ok(data) => {
            return success_response(data);
        }
        Err(e) => {
            return error_response(e.to_string());
        }
    };
}

pub async fn kv_get(State(state): State<HttpServerState>) -> String {
    let kvs = state.kvs.read().await;
    let key = "k1".to_string();
    let value = kvs.get(&key);

    return success_response(value);
}
