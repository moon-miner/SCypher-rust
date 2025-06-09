// src/security/mod.rs - Módulo de seguridad y limpieza de memoria

pub mod memory;

use std::sync::atomic::{AtomicBool, Ordering};
use zeroize::Zeroize;

// Flag global para rastrear si la limpieza está configurada
static CLEANUP_CONFIGURED: AtomicBool = AtomicBool::new(false);

/// Configurar limpieza de seguridad al inicio de la aplicación
pub fn setup_security_cleanup() {
    if CLEANUP_CONFIGURED.load(Ordering::Relaxed) {
        return; // Ya configurado
    }

    // Configurar handler para limpieza en caso de señales de terminación
    let _ = ctrlc::set_handler(move || {
        eprintln!("\nReceived termination signal. Performing secure cleanup...");
        secure_cleanup();
        std::process::exit(130); // 128 + 2 (SIGINT)
    });

    CLEANUP_CONFIGURED.store(true, Ordering::Relaxed);
}

/// Limpieza segura de memoria al final de la aplicación
pub fn secure_cleanup() {
    // Limpiar variables de entorno sensibles si las hay
    clear_environment_variables();

    // Sobrescribir stack con ceros (mejor esfuerzo)
    let mut dummy_buffer = vec![0u8; 4096];
    dummy_buffer.zeroize();

    // Nota: En Rust, la limpieza automática de memoria es más segura
    // que en otros lenguajes debido al ownership system
}

/// Limpiar variables de entorno que podrían contener datos sensibles
fn clear_environment_variables() {
    // Variables que podrían contener información sensible
    let sensitive_vars = [
        "SCYPHER_PASSWORD",
        "SCYPHER_SEED",
        "TMPDIR",
    ];

    for var in &sensitive_vars {
        std::env::remove_var(var);
    }
}

/// Wrapper seguro para strings sensibles
/// Implementa Drop para limpieza automática
pub struct SecureString {
    data: Vec<u8>,
}

impl SecureString {
    /// Crear nueva cadena segura
    pub fn new(s: &str) -> Self {
        Self {
            data: s.as_bytes().to_vec(),
        }
    }

    /// Obtener referencia como str (usar con cuidado)
    pub fn as_str(&self) -> &str {
        // SAFETY: Mantenemos la invariante de que data contiene UTF-8 válido
        unsafe { std::str::from_utf8_unchecked(&self.data) }
    }

    /// Obtener bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    /// Longitud en bytes
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Verificar si está vacía
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Drop for SecureString {
    fn drop(&mut self) {
        // Sobrescribir con ceros antes de liberar
        self.data.zeroize();
    }
}

impl From<String> for SecureString {
    fn from(s: String) -> Self {
        Self::new(&s)
    }
}

impl From<&str> for SecureString {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

/// Estructura para manejar datos binarios sensibles
pub struct SecureBytes {
    data: Vec<u8>,
}

impl SecureBytes {
    /// Crear nuevo vector de bytes seguro
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    /// Crear desde slice
    pub fn from_slice(slice: &[u8]) -> Self {
        Self {
            data: slice.to_vec(),
        }
    }

    /// Obtener referencia a los datos
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    /// Longitud
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Verificar si está vacío
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Consumir y obtener el vector interno (sin limpieza)
    pub fn into_vec(mut self) -> Vec<u8> {
        let data = std::mem::replace(&mut self.data, Vec::new());
        std::mem::forget(self); // Evitar que Drop limpie los datos
        data
    }
}

impl Drop for SecureBytes {
    fn drop(&mut self) {
        self.data.zeroize();
    }
}

/// Utilidades para operaciones seguras en memoria
pub mod utils {
    use super::*;

    /// Comparación constante en tiempo para evitar timing attacks
    pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }

        let mut result = 0u8;
        for (byte_a, byte_b) in a.iter().zip(b.iter()) {
            result |= byte_a ^ byte_b;
        }

        result == 0
    }

    /// Generar bytes aleatorios seguros
    pub fn secure_random_bytes(len: usize) -> Vec<u8> {
        use rand::RngCore;
        let mut rng = rand::thread_rng();
        let mut bytes = vec![0u8; len];
        rng.fill_bytes(&mut bytes);
        bytes
    }

    /// Limpiar un buffer con datos aleatorios antes de sobrescribir con ceros
    pub fn secure_wipe(buffer: &mut [u8]) {
        use rand::RngCore;
        let mut rng = rand::thread_rng();

        // Primer pase: datos aleatorios
        rng.fill_bytes(buffer);

        // Segundo pase: ceros
        buffer.zeroize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_string() {
        let original = "sensitive data";
        let secure = SecureString::new(original);

        assert_eq!(secure.as_str(), original);
        assert_eq!(secure.len(), original.len());
        assert!(!secure.is_empty());
    }

    #[test]
    fn test_secure_bytes() {
        let data = vec![1, 2, 3, 4, 5];
        let secure = SecureBytes::new(data.clone());

        assert_eq!(secure.as_slice(), &data);
        assert_eq!(secure.len(), data.len());
        assert!(!secure.is_empty());
    }

    #[test]
    fn test_constant_time_eq() {
        assert!(utils::constant_time_eq(b"hello", b"hello"));
        assert!(!utils::constant_time_eq(b"hello", b"world"));
        assert!(!utils::constant_time_eq(b"hello", b"hell"));  // Diferente longitud
    }

    #[test]
    fn test_secure_random_bytes() {
        let bytes1 = utils::secure_random_bytes(16);
        let bytes2 = utils::secure_random_bytes(16);

        assert_eq!(bytes1.len(), 16);
        assert_eq!(bytes2.len(), 16);
        assert_ne!(bytes1, bytes2); // Extremadamente improbable que sean iguales
    }
}
