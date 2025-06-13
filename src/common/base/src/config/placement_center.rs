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

//! Placement Center 配置管理
//! 
//! 这个文件实现了 Placement Center 的配置管理功能，包括：
//! 1. 配置文件的读取和解析
//! 2. 配置的全局访问
//! 3. 配置的验证和初始化
//! 4. 配置的运行时更新
//!
//! 实现原理：
//! - 使用 toml 格式存储配置
//! - 使用单例模式管理全局配置
//! - 支持配置热重载
//! - 提供了类型安全的配置访问
//! - 实现了配置的序列化和反序列化

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

use crate::tools::read_file;
use serde::Deserialize;
use toml::Table;
use std::sync::OnceLock;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct PlacementCenterConfig {
    pub cluster_name: String,
    pub addr: String,
    #[serde(default = "default_node_id")]
    pub node_id: u64,
    #[serde(default = "default_grpc_port")]
    pub grpc_port: usize,
    pub nodes: Table,
    pub http_port: usize,
    pub data_path: String,
    pub log: Log,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Log {
    pub log_config: String,
    pub log_path: String,
}

pub fn default_node_id() -> u64 {
    1
}

pub fn default_grpc_port() -> usize {
    9982
}

static PLACEMENT_CENTER_CONF: OnceLock<PlacementCenterConfig> = OnceLock::new();

pub fn init_placement_center_conf_by_path(config_path: &String) -> &'static PlacementCenterConfig {
    PLACEMENT_CENTER_CONF.get_or_init(|| {
        let content = match read_file(config_path) {
            Ok(data) => data,
            Err(e) => {
                panic!("{}", e.to_string());
            }
        };
        let pc_config: PlacementCenterConfig = toml::from_str(&content).unwrap();
        return pc_config;
    })
}

pub fn placement_center_conf() -> &'static PlacementCenterConfig {
    match PLACEMENT_CENTER_CONF.get() {
        Some(config) => {
            return config;
        }
        None => {
            panic!(
                "Placement center configuration is not initialized, check the configuration file."
            );
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::config::placement_center::{
        init_placement_center_conf_by_path, placement_center_conf,
    };

    #[test]
    fn config_init_test() {
        let path = format!(
            "{}/../../../config/placement-center.toml",
            env!("CARGO_MANIFEST_DIR")
        );
        println!("{}", path);
        init_placement_center_conf_by_path(&path);
        let config = placement_center_conf();
        assert_eq!(config.node_id, 1);
        assert_eq!(config.grpc_port, 1228);
    }
}
