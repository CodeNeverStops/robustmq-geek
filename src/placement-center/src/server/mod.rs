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

//! 服务模块总入口
//!
//! 该文件为 Placement Center 服务相关功能的总入口，包含：
//! 1. HTTP 服务模块
//! 2. gRPC 服务模块
//!
//! 实现原理：
//! - 按功能模块划分子模块
//! - 统一对外暴露服务相关接口

pub mod http;
pub mod grpc;