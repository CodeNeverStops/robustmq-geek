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

//! HTTP 服务模块
//!
//! 该文件为 Placement Center HTTP 服务相关功能的模块入口，包含：
//! 1. index 路由
//! 2. OpenRaft 路由
//! 3. HTTP 服务器实现
//! 4. 路由工具函数
//!
//! 实现原理：
//! - 按功能模块划分子模块
//! - 统一对外暴露 HTTP 相关接口
//! - 支持 RESTful API 设计

pub mod index;
pub mod openraft;
pub mod server;

pub(crate) fn v1_path(path: &str) -> String {
    return format!("/v1{}", path);
}

pub(crate) fn path_create(path: &str) -> String {
    return format!("{}/create", path);
}

pub(crate) fn path_update(path: &str) -> String {
    return format!("{}/update", path);
}

pub(crate) fn path_delete(path: &str) -> String {
    return format!("{}/delete", path);
}

pub(crate) fn path_list(path: &str) -> String {
    return format!("{}/list", path);
}
