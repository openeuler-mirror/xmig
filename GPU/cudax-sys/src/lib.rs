#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(unnecessary_transmutes)]

pub mod driver {
    include!("driver/bindings.rs");
}

pub mod nccl {
    include!("nccl/bindings.rs");
}

pub mod runtime {
    include!("runtime/bindings.rs");
}

pub mod nvml {
    include!("nvml/bindings.rs");
}

pub mod cublas {
    include!("cublas/bindings.rs");
}

pub mod cublaslt {
    include!("cublaslt/bindings.rs");
}

pub mod bootstrap {
    include!("bootstrap/bindings.rs");
}