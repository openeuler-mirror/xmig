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

use std::{fs, env, path::{Path, PathBuf}};

const CUDA_HOME: &str = "/usr/local/cuda";          // if you DO NOT have the environment variable, replace it with your own path
const NCCL_SRC_HOME: &str = "/home/tomoki/nccl";    // if you DO NOT have the environment variable, replace it with your own path

fn generate_single_binding(
    header: &str,               // header file
    header_path: &Path,         // path to header file
    include_paths: &[&Path],    // possible include paths
    white_list: &[&str],        // allowlisted symbols
    output: &Path,              // output path for bindings.rs
    cxx_flag: bool              // whether to enable C++
) {
    // bindgen builder
    let mut builder = bindgen::Builder::default()
        .header(header_path.join(header).to_str().unwrap())
        .generate_inline_functions(true);

    // add include paths
    builder = builder.clang_args(include_paths.iter().map(|p| format!("-I{}", p.display())));

    // add white list
    builder = white_list.iter().fold(builder, |b, pat| {
        b.allowlist_function(pat)
        .allowlist_type(pat)
        .allowlist_var(pat)
    });

    // enable C++ support
    if cxx_flag {
        let gcc_ver: i32 = 11;
        let cpp_clang_args = [
            "-x".to_string(),
            "c++".to_string(),
            format!("-std=c++17"),
            format!("-I/usr/include/c++/{}", gcc_ver),
            format!("-I/usr/include/x86_64-linux-gnu/c++/{}", gcc_ver),
        ];

        builder = builder.clang_args(cpp_clang_args);
    }

    // bind
    let bindings = builder
        .generate()
        .expect("Unable to generate bindings");

    // output
    bindings
        .write_to_file(output)
        .expect("Couldn't write bindings!");
}

fn generate_all_bindings(include_paths: &[&Path]) {
    let cublas_out_dir = PathBuf::from("src/cublas");
    let nccl_out_dir = PathBuf::from("src/nccl");
    let cuda_runtime_out_dir = PathBuf::from("src/runtime");
    let nvml_out_dir = PathBuf::from("src/nvml");
    let cuda_out_dir = PathBuf::from("src/driver");
    let cublaslt_out_dir = PathBuf::from("src/cublaslt");
    let bootstrap_out_dir = PathBuf::from("src/bootstrap");
    
    fs::create_dir_all(&cublas_out_dir).unwrap();
    fs::create_dir_all(&nccl_out_dir).unwrap();
    fs::create_dir_all(&cuda_runtime_out_dir).unwrap();
    fs::create_dir_all(&nvml_out_dir).unwrap();
    fs::create_dir_all(&cuda_out_dir).unwrap();
    fs::create_dir_all(&cublaslt_out_dir).unwrap();
    fs::create_dir_all(&bootstrap_out_dir).unwrap();

    let system_include_path = Path::new("/usr/include");
    let cuda_include_path = Path::new(&env::var("CUDA_HOME")
        .unwrap_or(CUDA_HOME.to_string()))
        .join("include");
    let nccl_src_include_path = Path::new(&env::var("NCCL_SRC_HOME")
        .unwrap_or(NCCL_SRC_HOME.to_string()))
        .join("include");

    generate_single_binding(
        "cublas.h", 
        &cuda_include_path, 
        include_paths, 
        &["^cublas.*", "^CUBLAS.*"], 
        &cublas_out_dir.join("bindings.rs"),
        false,
    );
    generate_single_binding(
        "nccl.h", 
        &system_include_path, 
        include_paths, 
        &["^nccl.*", "^pnccl.*"],
        &nccl_out_dir.join("bindings.rs"),
        false,
    );
    generate_single_binding(
        "cuda_runtime.h", 
        &cuda_include_path, 
        include_paths, 
        &["^cuda.*"],
        &cuda_runtime_out_dir.join("bindings.rs"),
        false,
    );
    generate_single_binding(
        "nvml.h", 
        &cuda_include_path, 
        include_paths, 
        &["^nvml.*"],
        &nvml_out_dir.join("bindings.rs"),
        false,
    );
    generate_single_binding(
        "cuda.h", 
        &cuda_include_path, 
        include_paths, 
        &["^cu.*"],
        &cuda_out_dir.join("bindings.rs"),
        false,
    );
    generate_single_binding(
        "cublasLt.h", 
        &cuda_include_path, 
        include_paths, 
        &["^cublasLt.*"],
        &cublaslt_out_dir.join("bindings.rs"),
        false,
    );
    generate_single_binding(
        "bootstrap.h",
        &nccl_src_include_path,
        include_paths,
        &["^bootstrap.*", "ncclCalloc.*", "ncclComm", "ncclComm_t"],
        &bootstrap_out_dir.join("bindings.rs"),
        true,
    );
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cuda_home = env::var("CUDA_HOME")
            .unwrap_or(CUDA_HOME.to_string());
    let cuda_home = Path::new(&cuda_home);
    let nccl_src_home = env::var("NCCL_SRC_HOME")
            .unwrap_or(NCCL_SRC_HOME.to_string());
    let nccl_src_home = Path::new(&nccl_src_home);

    let include_paths = [
        Path::new("/usr/include"),
        &cuda_home.join("include"),
        &nccl_src_home.join("src").join("include"),
        &nccl_src_home.join("src").join("include").join("plugin"),
    ];

    generate_all_bindings(&include_paths);

    Ok(())
}