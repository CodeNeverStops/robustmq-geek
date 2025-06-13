//! OpenRaft 路由实现
//! 
//! 这个文件实现了 OpenRaft 的路由层，负责：
//! 1. 请求路由分发
//! 2. 请求数据封装
//! 3. 响应数据处理
//! 4. 路由规则管理
//!
//! 实现原理：
//! - 定义了请求和响应的数据结构
//! - 实现了请求的路由逻辑
//! - 支持请求的序列化和反序列化
//! - 提供了路由规则的配置接口
//! - 集成了错误处理机制

use serde::{Deserialize, Serialize};

/**
 * Here you will set the types of request that will interact with the raft nodes.
 * For example the `Set` will be used to write data (key and value) to the raft database.
 * The `AddNode` will append a new node to the current existing shared list of nodes.
 * You will want to add any request that can write data in all nodes here.
 */
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AppRequestData {
    Set { key: String, value: String },

    Delete { key: String },
}

/**
 * Here you will defined what type of answer you expect from reading the data of a node.
 * In this example it will return a optional value from a given key in
 * the `ExampleRequest.Set`.
 *
 * TODO: Should we explain how to create multiple `AppDataResponse`?
 *
 */
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppResponseData {
    pub value: Option<String>,
}
