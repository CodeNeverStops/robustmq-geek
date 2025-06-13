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

//! OpenRaft 模块
//! 
//! 这个文件是 OpenRaft 框架的模块定义，包含以下组件：
//! 1. 网络层：处理节点间通信
//! 2. Raft 节点：实现 Raft 协议
//! 3. 路由：处理请求路由
//! 4. 存储：管理状态存储
//! 5. 类型配置：定义数据类型
//! 6. 错误处理：处理异常情况
//!
//! 实现原理：
//! - 模块化设计，每个组件独立
//! - 使用 OpenRaft 框架的核心功能
//! - 实现了自定义的网络层
//! - 提供了完整的错误处理机制

pub mod network;
pub mod raft_node;
pub mod route;
pub mod store;
pub mod typeconfig;
pub mod error;