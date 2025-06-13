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

//! 存储模块
//!
//! 该文件为存储相关功能的模块入口，包含：
//! 1. 存储引擎
//! 2. KV 存储
//! 3. RocksDB 封装
//! 4. Raft 协议存储
//! 5. 键生成规则
//! 6. 数据包装结构
//!
//! 实现原理：
//! - 按功能模块划分子模块
//! - 统一对外暴露存储相关接口
//! - 支持数据持久化与高可用

use common_base::tools::now_second;
use serde::{Deserialize, Serialize};

pub mod engine;
pub mod kv;
pub mod rocksdb;
pub mod raft;
pub mod keys;

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageDataWrap {
    pub data: Vec<u8>,
    pub create_time: u64,
}

impl StorageDataWrap {
    pub fn new(data: Vec<u8>) -> Self {
        return StorageDataWrap {
            data,
            create_time: now_second(),
        };
    }
}
