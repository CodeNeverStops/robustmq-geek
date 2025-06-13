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

//! Placement Center 日志管理
//! 
//! 这个文件实现了 Placement Center 的日志管理功能，包括：
//! 1. 日志系统的初始化
//! 2. 日志配置的加载
//! 3. 日志目录的管理
//! 4. 日志格式的定制
//!
//! 实现原理：
//! - 使用 log4rs 作为日志框架
//! - 支持 YAML 格式的日志配置
//! - 实现了日志文件的自动创建和轮转
//! - 支持不同级别的日志输出
//! - 提供了日志路径的动态配置

use crate::{
    config::placement_center::placement_center_conf,
    tools::{create_fold, file_exists, read_file},
};

pub fn init_placement_center_log() {
    let conf = placement_center_conf();
    if !file_exists(&conf.log.log_config) {
        panic!(
            "Logging configuration file {} does not exist",
            conf.log.log_config
        );
    }

    match create_fold(&conf.log.log_path) {
        Ok(()) => {}
        Err(_) => {
            panic!("Failed to initialize log directory {}", conf.log.log_path);
        }
    }

    let content = match read_file(&conf.log.log_config) {
        Ok(data) => data,
        Err(e) => {
            panic!("{}", e.to_string());
        }
    };

    let config_content = content.replace("{$path}", &conf.log.log_path);
    println!("{}","log config:");
    println!("{}", config_content);
    let config = match serde_yaml::from_str(&config_content) {
        Ok(data) => data,
        Err(e) => {
            panic!(
                "Failed to parse the contents of the config file {} with error message :{}",
                conf.log.log_config,
                e.to_string()
            );
        }
    };

    match log4rs::init_raw_config(config) {
        Ok(_) => {}
        Err(e) => {
            panic!("{}", e.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_print() {}
}
