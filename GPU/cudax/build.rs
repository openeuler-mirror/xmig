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

use std::{env, path::Path};

fn multiarch_path() -> &'static str {
    let dir = match env::consts::ARCH {
        "x86_64" => "x86_64-linux-gnu",
        "x86" => "i386-linux-gnu",
        "aarch64" => "aarch64-linux-gnu",
        _ => panic!("Unsupported architecture"),
    };

    dir
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cuda_home = env::var("CUDA_HOME").unwrap_or("/usr/local/cuda".to_string());
    let cuda_home = Path::new(&cuda_home);

    let lib_paths = [
        &cuda_home.join("lib64"),
        &cuda_home.join("lib64").join("stubs"),
        &cuda_home.join("lib64"),
        &cuda_home.join("lib64").join("stubs"),
        &Path::new(&format!("/usr/lib/{}", multiarch_path())).to_path_buf(),
        Path::new("/usr/lib64"),
        Path::new("/usr/lib64"),
        Path::new("/usr/lib/wsl/lib"),
    ];
    let lib_names = ["cublas", "nccl", "cudart", "nvidia-ml", "cuda", "cublasLt"];

    for lib in lib_names {
        let file_name = format!("lib{}.so", lib);
        let found = lib_paths.iter().find(|p| p.join(&file_name).exists());

        if let Some(path) = found {
            println!("cargo:rustc-link-search=native={}", path.display());
            println!("cargo:rustc-link-lib=dylib={}", lib);
        } else {
            panic!("Could not find {} in {:?}", file_name, lib_paths);
        }
    }

    Ok(())
}
