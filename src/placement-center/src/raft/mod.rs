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

//! Raft 模块
//!
//! 该文件为 Raft 共识算法相关模块的入口，包含：
//! 1. 日志应用
//! 2. 状态机实现
//! 3. 集群元数据
//! 4. 节点管理
//! 5. 存储实现
//! 6. 路由逻辑
//!
//! 实现原理：
//! - 按功能模块划分子模块
//! - 统一对外暴露 Raft 相关接口
//! - 支持分布式一致性与高可用

/*
 * Copyright (c) 2023 RobustMQ Team
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
pub mod apply;
pub mod machine;
pub mod metadata;
pub mod peer;
pub mod storage;
pub mod node;
pub mod route;