📋 Plan de Implementación SCypher-Rust por Etapas
🎯 Objetivo General
Implementar una versión completa en Rust de SCypher con interfaz CLI compatible con la versión Bash, modo silent para scripting, y protecciones de seguridad avanzadas para el manejo seguro de claves privadas de criptomonedas.
📐 Arquitectura Base (Ya Implementada ✅)

✅ Estructura modular completa (crypto, bip39, cli, security, error)
✅ Validación BIP39 básica
✅ Operaciones XOR con keystream Argon2id
✅ Limpieza segura de memoria con zeroize
✅ CLI básica con clap
✅ Manejo de errores centralizado


🚀 ETAPA 1: Interfaz CLI Completa y Modo Silent
Objetivo: Implementar compatibilidad completa con la interfaz del script Bash y modo silent para scripting.
Tareas Específicas:

Agregar argumentos CLI faltantes:
rust// En src/main.rs
- --silent / -s (modo scripting sin prompts)
- --license (mostrar licencia)
- --details (explicación detallada del cifrado)

Implementar modo silent:
rust// Nuevo: src/cli/silent.rs
- Lectura desde stdin sin prompts
- Salida solo a stdout/stderr según corresponda
- Manejo de pipes y redirección

Mejorar lectura de entrada:
rust// En src/cli/input.rs
- Detección automática archivo vs frase
- Lectura desde stdin en modo silent
- Validación con reintentos (solo modo interactivo)


Archivos Nuevos:

src/cli/silent.rs

Archivos a Modificar:

src/main.rs (argumentos CLI)
src/cli/input.rs (detección automática)
src/cli/mod.rs (re-exports)

Criterios de Éxito:

echo "phrase" | scypher-rust -s -i 5 -m 131072 funciona
scypher-rust --license muestra licencia
Detección automática de archivos vs frases


🖥️ ETAPA 2: Sistema de Menús Interactivo
Objetivo: Implementar el sistema completo de menús igual al script Bash con banner ASCII y navegación.
Tareas Específicas:

Crear sistema de menús:
rust// Nuevo: src/cli/menu.rs
- Menú principal con banner ASCII
- Submenú ayuda/licencia/detalles
- Menú post-procesamiento
- Navegación entre menús

Implementar pantallas informativas:
rust// Nuevo: src/cli/display.rs
- Banner ASCII de SCypher
- Texto de licencia y disclaimer
- Explicación detallada del proceso XOR
- Ejemplos de uso

Integrar flujo de menús:
rust// En src/main.rs
- Lógica de cuándo mostrar menús
- Manejo de opciones post-procesamiento
- Loop principal para reintentos


Archivos Nuevos:

src/cli/menu.rs
src/cli/display.rs

Archivos a Modificar:

src/main.rs (integración de menús)
src/cli/mod.rs (re-exports)

Criterios de Éxito:

Menú interactivo idéntico al script Bash
Banner ASCII se muestra correctamente
Navegación fluida entre todas las pantallas


🔐 ETAPA 3: Protecciones de Seguridad Avanzadas
Objetivo: Implementar protecciones de seguridad específicas para el manejo de claves privadas.
Tareas Específicas:

Protecciones de memoria:
rust// En src/security/memory.rs
- Memory locking (mlock) para datos sensibles
- Prevención de swap de páginas críticas
- Verificación de integridad de memoria

Protecciones de proceso:
rust// Nuevo: src/security/process.rs
- Disable core dumps
- Anti-debugging (ptrace protection)
- Process isolation donde sea posible

Protecciones de entorno:
rust// Nuevo: src/security/environment.rs
- Limpieza de variables de entorno sensibles
- Configuración de umask seguro
- Validación de entorno de ejecución


Archivos Nuevos:

src/security/process.rs
src/security/environment.rs

Archivos a Modificar:

src/security/memory.rs (funciones adicionales)
src/security/mod.rs (re-exports)
src/main.rs (inicialización de protecciones)

Criterios de Éxito:

Memoria sensible está bloqueada y no puede ir a swap
Core dumps deshabilitados
Protección contra debugging básico


🔍 ETAPA 4: Validación Avanzada y UX Mejorada
Objetivo: Mejorar la experiencia de usuario con validación inteligente y manejo de errores.
Tareas Específicas:

Validación mejorada con sugerencias:
rust// En src/bip39/validation.rs (usar funciones existentes)
- Sugerencias de corrección para palabras inválidas (ya existe find_closest_word)
- Múltiples intentos con contexto conservado
- Mensajes de error más informativos

Manejo de archivos robusto:
rust// En src/cli/input.rs
- Detección inteligente de tipos de entrada
- Validación de permisos y existencia
- Manejo de errores de E/O descriptivo

Mejoras en salida:
rust// En src/cli/output.rs (usar funciones existentes)
- Formateo visual de seed phrases
- Confirmaciones de sobreescritura
- Progreso visual para operaciones largas


Archivos a Modificar:

src/bip39/validation.rs (usar funciones existentes)
src/cli/input.rs (mejoras de detección)
src/cli/output.rs (mejoras visuales)
src/error.rs (mensajes más descriptivos)

Criterios de Éxito:

Sugerencias automáticas para palabras BIP39 incorrectas
Detección inteligente de archivos vs frases
Interfaz visual pulida y profesional


🎨 ETAPA 5: Preparación para GUI y Optimizaciones
Objetivo: Refactorizar la lógica core para soportar GUI futuro y optimizar rendimiento.
Tareas Específicas:

Refactoring para GUI:
rust// En src/lib.rs
- API pública limpia para GUI
- Separación de lógica UI vs core
- Callbacks para progreso de operaciones

Optimizaciones de rendimiento:
rust// En src/crypto/
- Optimización de operaciones Argon2id
- Paralelización donde sea seguro
- Reducción de allocaciones temporales

Testing comprehensivo:
rust// Tests en todos los módulos
- Tests unitarios completos
- Tests de integración CLI
- Benchmarks de rendimiento


Archivos a Modificar:

src/lib.rs (API pública mejorada)
src/crypto/ (optimizaciones)
Todos los módulos (tests)

Criterios de Éxito:

API pública limpia y bien documentada
Tests cubren >90% del código
Rendimiento optimizado para operaciones críticas


📚 Información de Contexto para IA
Arquitectura Actual:

Lenguaje: Rust 2021 edition
Dependencies: clap, argon2, hex, sha2, rpassword, zeroize, ctrlc
Estructura: Modular con separación clara de responsabilidades
Estado: Compila y funciona básicamente, pero genera 44 warnings de código no usado

Decisiones de Diseño:

Seguridad first: Maneja claves privadas de crypto, máxima seguridad
No archivos temporales: Todo en memoria con limpieza segura
Compatibilidad Bash: Interfaz CLI idéntica al script original
XOR + Argon2id: Reemplaza SHAKE-256 del script Bash

Scripts de Referencia:

Bash original: SCypherV2.sh (incluido en documentos)
Funcionalidad core: Ya implementada en Rust
Falta: Interfaz completa, menús, modo silent, seguridad avanzada

Comandos de Desarrollo:
bashcargo build --release    # Compilar optimizado
cargo test               # Ejecutar tests
cargo run -- --help     # Probar CLI

🚨 Notas Importantes

Cada etapa debe compilar y funcionar antes de pasar a la siguiente
Preservar toda la funcionalidad existente al hacer cambios
Los warnings son normales hasta completar las etapas (código preparado pero no usado)
Seguridad es prioritaria sobre performance o comodidad
Mantener compatibilidad con script Bash en interfaz CLI


Este plan está diseñado para ser usado por cualquier IA de asistencia de código. Cada etapa es autocontenida y puede ser implementada independientemente con la información de contexto proporcionada.
