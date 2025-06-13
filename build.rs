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

//! 构建脚本
//! 
//! 这个文件是 Cargo 的构建脚本，负责：
//! 1. 构建前的准备工作
//! 2. 生成构建信息
//! 3. 处理构建依赖
//! 4. 配置构建环境
//!
//! 实现原理：
//! - 在 Cargo 构建过程中执行
//! - 可以访问环境变量和构建配置
//! - 可以生成代码和配置文件
//! - 可以检查系统依赖
//! - 可以自定义构建过程

fn main() {
    println("build.rs running");
}