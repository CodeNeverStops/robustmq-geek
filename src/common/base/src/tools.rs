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

//! 通用工具函数
//! 
//! 这个文件提供了一系列通用工具函数，包括：
//! 1. 文件操作：创建目录、检查文件存在性、读取文件
//! 2. 时间处理：获取当前时间戳
//! 3. 路径处理：路径拼接和检查
//!
//! 实现原理：
//! - 使用标准库的文件系统操作
//! - 提供了错误处理机制
//! - 实现了跨平台的路径处理
//! - 使用系统时间 API

use std::{
    fs,
    path::{self, Path}, time::{SystemTime, UNIX_EPOCH},
};

use crate::errors::RobustMQError;

pub fn create_fold(fold: &String) -> Result<(), RobustMQError> {
    if !Path::new(fold).exists() {
        fs::create_dir_all(fold)?
    }
    return Ok(());
}

pub fn file_exists(path: &String) -> bool {
    return Path::new(path).exists();
}

pub fn read_file(path: &String) -> Result<String, RobustMQError> {
    if !path::Path::new(path).exists() {
        return Err(RobustMQError::CommmonError(format!(
            "File {} does not exist",
            path
        )));
    }

    return Ok(fs::read_to_string(&path)?);
}

pub fn now_second() -> u64 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
}