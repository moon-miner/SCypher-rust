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

/// Muestra la licencia y disclaimer
fn show_license() {
    println!(r#"
SCypher v{} - License and Disclaimer

License:
This project is released under the MIT License. You are free to:
- Use the software commercially
- Modify the source code
- Distribute the software
- Use it privately

Disclaimer:
THIS SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT
OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

The developers assume no responsibility for:
- Loss of funds or assets
- Incorrect usage of the software
- Modifications made by third parties
- Security implications of usage in specific contexts
- Possible malfunction of the software

⚠️  CRITICAL SECURITY WARNING:
This software handles cryptocurrency seed phrases. Always:
- Test with non-critical phrases first
- Keep secure backups of original seeds
- Use strong, unique passwords
- Verify results before using with real funds
"#, VERSION);
}

/// Muestra explicación detallada del proceso XOR
fn show_details() {
    println!(r#"
SCypher v{} - Technical Details

How XOR-Based Seed Encryption Works:

1. Core Concept - XOR Encryption:
   • XOR (exclusive OR) is a reversible binary operation
   • When you XOR data twice with the same key, you get back the original
   • Formula: (data XOR key) XOR key = data

2. The Process:
   Encryption:
   • Your seed phrase is converted to binary (11 bits per word)
   • Your password generates a keystream using Argon2id key derivation
   • Multiple iterations strengthen the keystream against attacks
   • Binary seed XOR keystream = encrypted binary
   • Encrypted binary is converted back to valid BIP39 words
   • The checksum is recalculated to ensure BIP39 validity

   Decryption:
   • The encrypted phrase is converted to binary
   • Same password and iterations generate the identical keystream
   • Encrypted binary XOR keystream = original binary
   • Original binary is converted back to your seed phrase

3. Security Features:
   • Argon2id memory-hard key derivation function
   • Iterations add computational cost for attackers
   • XOR provides perfect secrecy with secure keystream
   • Output is always a valid BIP39 phrase with correct checksum
   • No statistical patterns in encrypted output
   • Secure memory cleanup prevents data leaks

4. Checksum Handling:
   • BIP39 phrases include a checksum for error detection
   • After XOR encryption, we recalculate the checksum
   • This ensures compatibility with all BIP39-compliant wallets
   • The recalculation is deterministic and preserves security

5. Important Security Notes:
   • Same password + iterations always produces same result
   • Use strong, unique passwords for maximum security
   • More iterations = more security but slower processing
   • Test with non-critical phrases first
   • Keep secure backups of original seeds

Technical Implementation:
- Language: Rust (memory-safe, secure)
- Key Derivation: Argon2id (memory-hard, GPU-resistant)
- XOR Implementation: Bit-level operations
- Memory Management: Automatic cleanup with zeroize
- BIP39 Compliance: Full wordlist validation and checksum verification
"#, VERSION);
}

fn main() {
    // ======= NUEVAS PROTECCIONES DE SEGURIDAD =======
    // Configurar protecciones comprehensivas de seguridad al inicio
    if let Err(e) = security::setup_comprehensive_security() {
        eprintln!("Warning: Could not configure all security protections: {}", e);
        eprintln!("Continuing with reduced security...");
    }

    // Realizar auditoría de seguridad
    let security_report = security::security_audit();

    // Mostrar reporte de seguridad si hay problemas
    if security_report.has_critical_issues() {
        eprintln!("SECURITY AUDIT REPORT:");
        eprintln!("{}", security_report.generate_report());

        // En modo release, terminar si hay problemas críticos
        #[cfg(not(debug_assertions))]
        {
            eprintln!("Critical security issues detected. Terminating for safety.");
            process::exit(1);
        }

        // En modo debug, solo advertir
        #[cfg(debug_assertions)]
        {
            eprintln!("Warning: Critical security issues detected, but continuing in debug mode.");
        }
    } else if !security_report.warnings().is_empty() || !security_report.info().is_empty() {
        // Mostrar advertencias e información solo si no hay problemas críticos
        eprintln!("Security status:");
        for warning in security_report.warnings() {
            eprintln!("  ⚠️  {}", warning);
        }
        for info in security_report.info() {
            eprintln!("  ℹ️  {}", info);
        }
    }
    // ======= FIN DE PROTECCIONES DE SEGURIDAD =======

    // Configurar limpieza segura de memoria al salir (ESTA LÍNEA YA EXISTÍA)
    security::setup_security_cleanup();

    // Configurar CLI usando clap
    let matches = Command::new("SCypher")
        .version(VERSION)
        .about("XOR-based BIP39 seed cipher with Argon2id key derivation")
        .long_about("SCypher provides secure, reversible transformation of BIP39 seed phrases \
                    using XOR encryption with Argon2id key derivation. The same operation \
                    performs both encryption and decryption due to XOR's symmetric nature.")

        // Verificar argumentos especiales primero
        .arg(Arg::new("license")
            .long("license")
            .help("Show license and disclaimer")
            .action(clap::ArgAction::SetTrue))

        .arg(Arg::new("details")
            .long("details")
            .help("Show detailed explanation of the XOR cipher process")
            .action(clap::ArgAction::SetTrue))

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

        .arg(Arg::new("silent")
            .short('s')
            .long("silent")
            .help("Silent mode - no prompts, reads from stdin (for scripting)")
            .action(clap::ArgAction::SetTrue))

        .get_matches();

    // Verificar argumentos especiales antes del procesamiento principal
    if matches.get_flag("license") {
        show_license();
        return;
    }

    if matches.get_flag("details") {
        show_details();
        return;
    }



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

/// Función helper para verificar si clap::ArgMatches tiene argumentos presentes
trait ArgMatchesExt {
    fn args_present(&self) -> bool;
}

impl ArgMatchesExt for clap::ArgMatches {
    fn args_present(&self) -> bool {
        // Verificar si algún argumento fue proporcionado
        self.get_flag("decrypt") ||
        self.get_one::<String>("output").is_some() ||
        self.get_one::<String>("input-file").is_some() ||
        self.get_flag("skip-checksum") ||
        *self.get_one::<u32>("iterations").unwrap() != 5 ||  // Default value
        *self.get_one::<u32>("memory").unwrap() != 131072    // Default value
    }
}

/// Función principal que coordina toda la operación
fn run(matches: &clap::ArgMatches) -> Result<()> {
    // Verificar si hay argumentos CLI (modo no-interactivo)
    let has_cli_args = matches.args_present();

    // Si no hay argumentos CLI, ejecutar modo interactivo con menús
    if !has_cli_args {
        return run_interactive_mode();
    }

    // Modo CLI tradicional
    run_cli_mode(matches)
}

/// Ejecutar modo interactivo con sistema de menús
fn run_interactive_mode() -> Result<()> {
    loop {
        // Mostrar menú y obtener estado
        let menu_state = cli::run_interactive_menu()?;

        if menu_state.should_exit {
            break;
        }

        if menu_state.return_to_main {
            // Usuario eligió "Encrypt/Decrypt", ejecutar procesamiento interactivo
            if let Err(e) = run_interactive_processing() {
                cli::handle_menu_error(&e.to_string());
                continue;
            }
        }
    }

    Ok(())
}

/// Ejecutar procesamiento interactivo (desde menú)
fn run_interactive_processing() -> Result<()> {
    // Valores por defecto para modo interactivo
    let iterations = 5u32;
    let memory_cost = 131072u32;

    cli::clear_screen();

    // Mostrar información del modo
    println!("{}SCypher v{} - Interactive Processing Mode{}",
             cli::colors::BRIGHT, VERSION, cli::colors::RESET);
    println!("{}Security: Argon2id with {} iterations, {}KB memory{}\n",
             cli::colors::DIM, iterations, memory_cost, cli::colors::RESET);

    // 1. Obtener frase semilla de forma interactiva
    let seed_phrase = cli::read_seed_interactive(false)?;

    // 2. Validar formato BIP39
    println!("Validating BIP39 format...");
    bip39::validate_seed_phrase_complete(&seed_phrase)?;
    println!("{}✓ Seed phrase format is valid{}\n", cli::colors::SUCCESS, cli::colors::RESET);

    // 3. Obtener contraseña de forma segura
    let password = cli::read_password_secure()?;

    // 4. Realizar transformación XOR
    println!("Processing with Argon2id key derivation...");
    let result = crypto::transform_seed(&seed_phrase, &password, iterations, memory_cost)?;

    // 5. Verificar resultado
    match bip39::verify_checksum(&result) {
        Ok(true) => println!("{}✓ Result has valid BIP39 checksum{}", cli::colors::SUCCESS, cli::colors::RESET),
        Ok(false) => println!("{}⚠️  Result checksum is invalid - check password and input{}", cli::colors::WARNING, cli::colors::RESET),
        Err(_) => println!("{}⚠️  Could not verify result checksum{}", cli::colors::WARNING, cli::colors::RESET),
    }

    // 6. Mostrar resultado
    println!();
    println!("{}Result:{}", cli::colors::SUCCESS, cli::colors::RESET);
    println!("─────────────────────────────────────────────────────────────");
    println!("{}{}{}", cli::colors::PRIMARY, result, cli::colors::RESET);
    println!("─────────────────────────────────────────────────────────────");

    // 7. Manejar menú post-procesamiento
    let should_exit = cli::handle_post_processing_menu(&result)?;

    if should_exit {
        println!("{}✓ Operation completed successfully{}", cli::colors::SUCCESS, cli::colors::RESET);
        security::secure_cleanup();
        std::process::exit(0);
    }

    Ok(())
}

/// Ejecutar modo CLI tradicional (con argumentos)
fn run_cli_mode(matches: &clap::ArgMatches) -> Result<()> {
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
    cli::clear_screen();
    cli::show_banner();

    println!("{}SECURITY FEATURES:{}", cli::colors::SUCCESS, cli::colors::RESET);
    println!("• XOR encryption with perfect reversibility");
    println!("• Argon2id memory-hard key derivation");
    println!("• BIP39 checksum preservation");
    println!("• Secure memory cleanup");
    println!("• No network access required");
    println!();

    println!("{}USAGE EXAMPLES:{}", cli::colors::PRIMARY, cli::colors::RESET);
    println!("  scypher-rust                           # Interactive mode with menus");
    println!("  scypher-rust -d                        # Decryption mode (same as encryption)");
    println!("  scypher-rust -i 10 -m 262144          # Higher security (10 iter, 256MB)");
    println!("  scypher-rust -f input.txt -o result   # File input/output");
    println!("  scypher-rust --skip-checksum          # Skip validation (not recommended)");
    println!();

    println!("{}SECURITY PARAMETERS:{}", cli::colors::WARNING, cli::colors::RESET);
    println!("• Iterations: Higher = more CPU time for attackers (1-100)");
    println!("• Memory: Higher = more RAM needed for attacks (8MB-2GB)");
    println!("• Recommended: 5 iterations, 128MB memory");
    println!();

    println!("{}The same operation encrypts and decrypts due to XOR symmetry.{}", cli::colors::DIM, cli::colors::RESET);
    println!("{}Use strong, unique passwords for maximum security.{}", cli::colors::DIM, cli::colors::RESET);
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
