// src/main.rs - Punto de entrada principal de SCypher

use clap::{Arg, Command};
use std::process;

// Declaración de módulos
mod crypto;
mod bip39;
mod cli;
mod security;
mod error;

// Importaciones
use crate::error::{SCypherError, Result};

const VERSION: &str = "3.0";
const DEFAULT_ITERATIONS: &str = "5";
const DEFAULT_MEMORY_COST: &str = "131072"; // 128MB en KB

fn main() {
    // Configurar limpieza segura de memoria al salir
    security::setup_security_cleanup();

    // Configurar CLI usando clap
    let matches = Command::new("SCypher")
        .version(VERSION)
        .about("XOR-based BIP39 seed cipher with Argon2id key derivation")
        .long_about("SCypher provides secure, reversible transformation of BIP39 seed phrases \
                    using XOR encryption with Argon2id key derivation. The same operation \
                    performs both encryption and decryption due to XOR's symmetric nature.")

        // Modo de operación (encrypt/decrypt son conceptualmente lo mismo pero útiles para claridad)
        .arg(Arg::new("encrypt")
            .short('e')
            .long("encrypt")
            .help("Encryption mode (default - same as decrypt due to XOR symmetry)")
            .action(clap::ArgAction::SetTrue))

        .arg(Arg::new("decrypt")
            .short('d')
            .long("decrypt")
            .help("Decryption mode (same as encrypt due to XOR symmetry)")
            .action(clap::ArgAction::SetTrue))

        // Archivo de salida
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .value_name("FILE")
            .help("Save output to file (will add .txt extension if needed)")
            .value_parser(clap::value_parser!(String)))

        // Parámetros de seguridad Argon2id
        .arg(Arg::new("iterations")
            .short('i')
            .long("iterations")
            .value_name("NUMBER")
            .help("Argon2id iterations (default: 5, min: 1, recommended: 3-10)")
            .default_value(DEFAULT_ITERATIONS)
            .value_parser(clap::value_parser!(u32)))

        .arg(Arg::new("memory")
            .short('m')
            .long("memory")
            .value_name("KB")
            .help("Argon2id memory cost in KB (default: 131072 = 128MB)")
            .default_value(DEFAULT_MEMORY_COST)
            .value_parser(clap::value_parser!(u32)))

        // Archivo de entrada
        .arg(Arg::new("input-file")
            .short('f')
            .long("file")
            .value_name("FILE")
            .help("Read seed phrase from file instead of interactive input")
            .value_parser(clap::value_parser!(String)))

        // Verificación de checksum
        .arg(Arg::new("skip-checksum")
            .long("skip-checksum")
            .help("Skip BIP39 checksum verification (not recommended)")
            .action(clap::ArgAction::SetTrue))

        .get_matches();

    // Ejecutar la aplicación y manejar errores
    if let Err(e) = run(&matches) {
        eprintln!("Error: {}", e);

        // Diferentes códigos de salida para diferentes tipos de error
        let exit_code = match e {
            SCypherError::InvalidSeedPhrase |
            SCypherError::InvalidWordCount(_) |
            SCypherError::InvalidBip39Word(_) |
            SCypherError::InvalidChecksum => 2,           // Errores de validación

            SCypherError::InvalidPassword |
            SCypherError::PasswordMismatch => 3,          // Errores de contraseña

            SCypherError::IoError(_) |
            SCypherError::FileError(_) => 4,              // Errores de E/O

            SCypherError::CryptoError(_) |
            SCypherError::KeyDerivationFailed => 5,       // Errores criptográficos

            _ => 1,                                       // Error general
        };

        process::exit(exit_code);
    }

    // Limpieza segura antes de salir
    security::secure_cleanup();
}

/// Función principal que coordina toda la operación
fn run(matches: &clap::ArgMatches) -> Result<()> {
    // Extraer argumentos
    let is_decrypt_mode = matches.get_flag("decrypt");
    let output_file = matches.get_one::<String>("output");
    let input_file = matches.get_one::<String>("input-file");
    let skip_checksum = matches.get_flag("skip-checksum");

    // Obtener parámetros de seguridad
    let iterations = *matches.get_one::<u32>("iterations").unwrap();
    let memory_cost = *matches.get_one::<u32>("memory").unwrap();

    // Validar parámetros
    validate_crypto_params(iterations, memory_cost)?;

    // Mostrar modo de operación (solo informativo, XOR es simétrico)
    let mode_name = if is_decrypt_mode { "Decryption" } else { "Encryption" };
    println!("SCypher v{} - {} Mode", VERSION, mode_name);
    println!("Security: Argon2id with {} iterations, {}KB memory\n", iterations, memory_cost);

    // 1. Obtener frase semilla
    let seed_phrase = if let Some(file_path) = input_file {
        cli::read_seed_from_file(file_path)?
    } else {
        cli::read_seed_interactive(is_decrypt_mode)?
    };

    // 2. Validar formato BIP39
    if !skip_checksum {
        println!("Validating BIP39 format...");
        bip39::validate_seed_phrase_complete(&seed_phrase)?;
        println!("✓ Seed phrase format is valid\n");
    } else {
        println!("⚠️  Skipping BIP39 validation (not recommended)\n");
    }

    // 3. Obtener contraseña de forma segura
    let password = cli::read_password_secure()?;

    // 4. Realizar transformación XOR
    println!("Processing with Argon2id key derivation...");
    let result = crypto::transform_seed(&seed_phrase, &password, iterations, memory_cost)?;

    // 5. Verificar resultado si es modo descifrado
    if is_decrypt_mode && !skip_checksum {
        match bip39::verify_checksum(&result) {
            Ok(true) => println!("✓ Result has valid BIP39 checksum"),
            Ok(false) => println!("⚠️  Result checksum is invalid - check password and input"),
            Err(_) => println!("⚠️  Could not verify result checksum"),
        }
    }

    // 6. Mostrar y guardar resultado
    cli::output_result(&result, output_file)?;

    println!("\n✓ Operation completed successfully");
    Ok(())
}

/// Validar que los parámetros criptográficos estén en rangos seguros
fn validate_crypto_params(iterations: u32, memory_cost: u32) -> Result<()> {
    // Validar iteraciones
    if iterations == 0 {
        return Err(SCypherError::InvalidIterations("0".to_string()));
    }

    if iterations > 100 {
        return Err(SCypherError::InvalidIterations(
            format!("{} (maximum recommended: 100)", iterations)
        ));
    }

    // Validar costo de memoria (mínimo 8MB, máximo 2GB)
    if memory_cost < 8192 {  // 8MB
        return Err(SCypherError::InvalidMemoryCost(
            format!("{}KB (minimum: 8192KB = 8MB)", memory_cost)
        ));
    }

    if memory_cost > 2_097_152 {  // 2GB
        return Err(SCypherError::InvalidMemoryCost(
            format!("{}KB (maximum: 2097152KB = 2GB)", memory_cost)
        ));
    }

    Ok(())
}

/// Mostrar información de ayuda extendida
fn show_extended_help() {
    println!(r#"
SCypher v{} - XOR-based BIP39 Seed Cipher

SECURITY FEATURES:
• XOR encryption with perfect reversibility
• Argon2id memory-hard key derivation
• BIP39 checksum preservation
• Secure memory cleanup
• No network access required

USAGE EXAMPLES:
  scypher-rust                           # Interactive mode
  scypher-rust -d                        # Decryption mode (same as encryption)
  scypher-rust -i 10 -m 262144          # Higher security (10 iter, 256MB)
  scypher-rust -f input.txt -o result   # File input/output
  scypher-rust --skip-checksum          # Skip validation (not recommended)

SECURITY PARAMETERS:
• Iterations: Higher = more CPU time for attackers (1-100)
• Memory: Higher = more RAM needed for attacks (8MB-2GB)
• Recommended: 5 iterations, 128MB memory

The same operation encrypts and decrypts due to XOR symmetry.
Use strong, unique passwords for maximum security.
"#, VERSION);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_crypto_params() {
        // Casos válidos
        assert!(validate_crypto_params(1, 8192).is_ok());
        assert!(validate_crypto_params(5, 131072).is_ok());
        assert!(validate_crypto_params(100, 2_097_152).is_ok());

        // Casos inválidos
        assert!(validate_crypto_params(0, 131072).is_err());
        assert!(validate_crypto_params(101, 131072).is_err());
        assert!(validate_crypto_params(5, 4096).is_err());     // Muy poca memoria
        assert!(validate_crypto_params(5, 3_000_000).is_err()); // Demasiada memoria
    }
}
