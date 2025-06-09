// src/cli/mod.rs - Módulo CLI principal

pub mod input;
pub mod output;

// Re-exportar funciones principales para fácil acceso
pub use input::{
    read_seed_interactive,
    read_seed_from_file,
    read_password_secure,
};

pub use output::{
    output_result,
    save_to_file,
};