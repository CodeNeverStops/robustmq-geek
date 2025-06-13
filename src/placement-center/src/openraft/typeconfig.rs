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

//! OpenRaft 类型配置
//! 
//! 这个文件定义了 OpenRaft 框架使用的类型配置，包括：
//! 1. 请求数据类型
//! 2. 响应数据类型
//! 3. 节点类型
//! 4. 快照数据类型
//!
//! 实现原理：
//! - 使用 OpenRaft 的类型系统
//! - 定义了自定义的数据类型
//! - 实现了序列化和反序列化
//! - 提供了类型安全的 API

use crate::openraft::raft_node::Node;
use crate::openraft::route::AppRequestData;
use crate::openraft::route::AppResponseData;
use std::io::Cursor;

pub type SnapshotData = Cursor<Vec<u8>>;

openraft::declare_raft_types!(
    pub TypeConfig:
        D = AppRequestData,
        R = AppResponseData,
        Node = Node,
);
