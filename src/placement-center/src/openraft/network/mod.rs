//! OpenRaft 网络层实现
//! 
//! 这个文件实现了 OpenRaft 的网络层，负责：
//! 1. 节点间通信
//! 2. RPC 请求处理
//! 3. 消息序列化和反序列化
//! 4. 网络错误处理
//!
//! 实现原理：
//! - 使用 tonic 实现 gRPC 通信
//! - 实现了 OpenRaft 的网络接口
//! - 使用连接池管理连接
//! - 支持异步操作
//! - 提供了重试机制

pub mod connection;
pub mod network;
