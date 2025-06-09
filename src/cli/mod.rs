// src/cli/mod.rs - Módulo CLI principal

pub mod input;
pub mod output;
pub mod silent;

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

pub use silent::{
    read_seed_from_stdin,
    read_password_from_stdin,
    read_iterations_from_stdin,
    detect_input_type,
    InputType,
};
