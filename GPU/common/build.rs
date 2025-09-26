// SPDX-License-Identifier: Mulan PSL v2
/*
 * Copyright (c) 2025 Huawei Technologies Co., Ltd.
 * This software is licensed under Mulan PSL v2.
 * You can use this software according to the terms and conditions of the Mulan PSL v2.
 * You may obtain a copy of Mulan PSL v2 at:
 *         http://license.coscl.org.cn/MulanPSL2
 *
 * THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND,
 * EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT,
 * MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
 * See the Mulan PSL v2 for more details.
 */

use std::{error::Error, path::PathBuf};

use walkdir::WalkDir;

const PROTO_DIR: &str = "proto";

fn main() -> Result<(), Box<dyn Error>> {
    let proto_files: Vec<PathBuf> = WalkDir::new(PROTO_DIR)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file() &&
            e.path().extension().map_or(false, |ext| ext == "proto")
        })
        .map(|e| e.into_path())
        .collect();

    prost_build::Config::new()
        .compile_protos(&proto_files, &[PROTO_DIR])?;

    for file in &proto_files {
        println!("cargo:rerun-if-changed={}", file.display());
    }
    println!("cargo:rerun-if-changed={}", PROTO_DIR);

    Ok(())
}