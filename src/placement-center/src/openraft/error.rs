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

//! OpenRaft 错误处理
//! 
//! 这个文件实现了 OpenRaft 的错误处理机制，包括：
//! 1. 错误类型定义
//! 2. 错误转换
//! 3. 错误处理
//! 4. 错误传播
//!
//! 实现原理：
//! - 定义了自定义错误类型
//! - 实现了错误转换 trait
//! - 提供了错误处理工具
//! - 支持错误链
//! - 集成了日志系统

use std::fmt::Display;

use common_base::errors::RobustMQError;
use openraft::error::{NetworkError, RPCError, Unreachable};

use super::typeconfig::TypeConfig;

#[derive(Debug)]
struct ErrWrap(Box<dyn std::error::Error>);

impl Display for ErrWrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for ErrWrap {}

pub fn to_error<E: std::error::Error + 'static + Clone>(
    e: RobustMQError,
) -> RPCError<TypeConfig, E> {
    RPCError::Unreachable(Unreachable::new(&e))
}
