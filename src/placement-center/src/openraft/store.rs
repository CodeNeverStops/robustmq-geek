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

//! OpenRaft 存储实现
//! 
//! 这个文件实现了 OpenRaft 的存储层，负责：
//! 1. 状态机存储
//! 2. 日志存储
//! 3. 快照管理
//! 4. 数据持久化
//!
//! 实现原理：
//! - 使用 RocksDB 作为底层存储
//! - 实现了 OpenRaft 的存储接口
//! - 支持数据的序列化和反序列化
//! - 提供了事务支持
//! - 实现了数据的一致性保证 