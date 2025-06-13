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

//! 协议定义库
//! 
//! 这个文件定义了项目使用的所有协议，包括：
//! 1. KV 存储协议
//! 2. Placement Center 协议
//! 3. 通用协议
//! 4. OpenRaft 协议
//!
//! 实现原理：
//! - 使用 Protocol Buffers 定义协议
//! - 使用 tonic 生成 gRPC 代码
//! - 支持序列化和反序列化
//! - 提供了类型安全的 API

pub mod kv;
pub mod placement;
pub mod common;
pub mod openraft;