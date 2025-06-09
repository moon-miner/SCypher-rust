//! Limpieza segura de memoria
//!
//! Este módulo proporciona utilidades para el manejo seguro de memoria,
//! incluyendo limpieza de datos sensibles y verificaciones de integridad.

use zeroize::Zeroize;

/// Limpiar buffer de memoria de forma segura
/// Sobrescribe con datos aleatorios antes de poner en ceros
pub fn secure_clear(buffer: &mut [u8]) {
    // Primer pase: datos aleatorios
    use rand::RngCore;
    rand::thread_rng().fill_bytes(buffer);

    // Segundo pase: ceros
    buffer.zeroize();
}

/// Verificar integridad de memoria básica
/// Retorna true si la memoria parece estar íntegra
pub fn check_memory_integrity() -> bool {
    // Test básico: allocar y verificar que podemos escribir/leer
    let mut test_buffer = vec![0u8; 1024];

    // Escribir patrón
    for (i, byte) in test_buffer.iter_mut().enumerate() {
        *byte = (i % 256) as u8;
    }

    // Verificar patrón
    let is_intact = test_buffer.iter().enumerate().all(|(i, &byte)| {
        byte == (i % 256) as u8
    });

    // Limpiar buffer de prueba
    secure_clear(&mut test_buffer);

    is_intact
}

/// Limpieza profunda de un vector
pub fn deep_clear_vec<T: Zeroize>(vec: &mut Vec<T>) {
    // Limpiar cada elemento
    for item in vec.iter_mut() {
        item.zeroize();
    }

    // Limpiar y reducir capacidad
    vec.clear();
    vec.shrink_to_fit();
}

/// Wrapper para strings que se autolimpian
pub struct SecureBuffer {
    data: Vec<u8>,
}

impl SecureBuffer {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0u8; size],
        }
    }

    pub fn from_slice(slice: &[u8]) -> Self {
        Self {
            data: slice.to_vec(),
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Drop for SecureBuffer {
    fn drop(&mut self) {
        secure_clear(&mut self.data);
    }
}

impl Zeroize for SecureBuffer {
    fn zeroize(&mut self) {
        self.data.zeroize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_clear() {
        let mut buffer = vec![0xFF; 32];
        secure_clear(&mut buffer);

        // Buffer debería estar lleno de ceros
        assert!(buffer.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_check_memory_integrity() {
        // Esta función siempre debería retornar true en condiciones normales
        assert!(check_memory_integrity());
    }

    #[test]
    fn test_deep_clear_vec() {
        let mut vec = vec![vec![1u8, 2, 3], vec![4, 5, 6]];
        deep_clear_vec(&mut vec);

        assert!(vec.is_empty());
        assert_eq!(vec.capacity(), 0); // shrink_to_fit debería reducir capacidad
    }

    #[test]
    fn test_secure_buffer() {
        let mut buffer = SecureBuffer::new(16);
        buffer.as_mut_slice().fill(0xFF);

        assert_eq!(buffer.len(), 16);
        assert!(!buffer.is_empty());
        assert!(buffer.as_slice().iter().all(|&b| b == 0xFF));

        // Al salir del scope, el drop debería limpiar automáticamente
    }

    #[test]
    fn test_secure_buffer_from_slice() {
        let data = b"sensitive data";
        let buffer = SecureBuffer::from_slice(data);

        assert_eq!(buffer.as_slice(), data);
        assert_eq!(buffer.len(), data.len());
    }
}
