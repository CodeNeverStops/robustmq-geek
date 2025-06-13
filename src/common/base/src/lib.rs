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

//! 基础库
//! 
//! 这个文件是项目的基础库，提供了以下功能：
//! 1. 配置管理
//! 2. 错误处理
//! 3. HTTP 响应处理
//! 4. 日志管理
//! 5. 通用工具函数
//!
//! 实现原理：
//! - 模块化设计，每个功能都是独立的模块
//! - 提供了统一的错误处理机制
//! - 实现了通用的 HTTP 响应格式
//! - 集成了日志系统
//! - 提供了常用的工具函数

pub mod config;
pub mod errors;
pub mod http_error;
pub mod http_response;
pub mod log;
pub mod tools;
