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

const CUDA_HOME: &str = "/usr/local/cuda";          // if you DO NOT have the environment variable, replace it with your own path
const NCCL_SRC_HOME: &str = "/usr/local/nccl";      // if you DO NOT have the environment variable, replace it with your own path

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
    println!("cargo:rustc-link-lib=dylib=stdc++");
    
    let cuda_home = env::var("CUDA_HOME")
            .unwrap_or(CUDA_HOME.to_string());
    let cuda_home = Path::new(&cuda_home);
    let nccl_src_home = env::var("NCCL_SRC_HOME")
            .unwrap_or(NCCL_SRC_HOME.to_string());
    let nccl_src_home = Path::new(&nccl_src_home);

    // dynamic library
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
    let lib_names = [
        "cublas",
        "nccl",
        "cudart",
        "nvidia-ml",
        "cuda",
        "cublasLt",
    ];

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

    // static library 
    let lib_paths = [
        nccl_src_home.join("build/lib"),
    ];
    let lib_names = [
        "nccl_static"
    ];
    for lib in lib_names {
        let file_name = format!("lib{}.a", lib);
        let found = lib_paths.iter().find(|p| p.join(&file_name).exists());

        if let Some(path) = found {
            println!("cargo:rustc-link-search=native={}", path.display());
            println!("cargo:rustc-link-lib=static={}", lib);
        } else {
            panic!("Could not find {} in {:?}", file_name, lib_paths);
        }
    }

    Ok(())
}