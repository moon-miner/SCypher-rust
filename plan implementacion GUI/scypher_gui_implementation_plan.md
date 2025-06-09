# SCypher GUI Implementation Plan

## 📋 Overview

This guide provides step-by-step instructions to implement a Tauri-based GUI for SCypher, a XOR-based BIP39 seed cipher written in Rust. The implementation transforms an existing CLI application into a professional desktop GUI with advanced seed input, address derivation, and maximum security features.

## 🎯 Project Context

- **Base Project**: SCypher-Rust (CLI application with complete Rust backend)
- **Target**: Desktop GUI application using Tauri (Rust backend + HTML/CSS/JS frontend)
- **Key Features**: Advanced word input system, BIP39 validation, address derivation, security modes
- **Reference**: Use provided maqueta v12 as visual and functional reference

## 📁 Project Structure

```
scypher-gui/
├── src-tauri/
│   ├── src/
│   │   ├── main.rs           # Tauri entry point
│   │   ├── commands.rs       # Tauri command handlers
│   │   ├── lib.rs           # Existing SCypher logic
│   │   ├── crypto/          # Existing crypto modules
│   │   ├── bip39/           # Existing BIP39 modules
│   │   ├── cli/             # Existing CLI modules (keep for reference)
│   │   └── security/        # Existing security modules
│   ├── tauri.conf.json      # Tauri configuration
│   └── Cargo.toml           # Rust dependencies
├── src/                     # Frontend
│   ├── index.html          # Main GUI
│   ├── styles.css          # Styling
│   ├── main.js            # Frontend logic
│   └── assets/            # Icons, images
└── README.md
```

## 🚀 PHASE 1: Tauri Setup and Integration (1-2 days)

### Step 1.1: Initialize Tauri Project

```bash
# Install Tauri CLI if not installed
cargo install tauri-cli

# Create new Tauri project
cargo tauri init

# Or convert existing project
# Copy your existing src/ directory to src-tauri/src/
```

### Step 1.2: Configure Cargo.toml

**File: `src-tauri/Cargo.toml`**

```toml
[package]
name = "scypher-gui"
version = "0.1.0"
edition = "2021"

[build-dependencies]
tauri-build = { version = "1.0", features = [] }

[dependencies]
tauri = { version = "1.0", features = ["api-all"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Your existing dependencies
clap = "4.0"           # Keep for CLI compatibility
argon2 = "0.5"         
hex = "0.4"            
sha2 = "0.10"          
rpassword = "7.0"      # May not be needed in GUI
zeroize = "1.6"        
ctrlc = "3.0"          # May not be needed in GUI
rand = "0.8"

[features]
default = ["custom-protocol"]
custom-protocol = ["tauri/custom-protocol"]
```

### Step 1.3: Configure Tauri Settings

**File: `src-tauri/tauri.conf.json`**

```json
{
  "build": {
    "beforeBuildCommand": "",
    "beforeDevCommand": "",
    "devPath": "../src",
    "distDir": "../src"
  },
  "package": {
    "productName": "SCypher",
    "version": "3.0.0"
  },
  "tauri": {
    "allowlist": {
      "all": false,
      "fs": {
        "all": false,
        "readFile": true,
        "writeFile": true,
        "readDir": false,
        "copyFile": false,
        "createDir": false,
        "removeDir": false,
        "removeFile": false,
        "renameFile": false,
        "exists": true
      },
      "dialog": {
        "all": false,
        "open": true,
        "save": true
      },
      "clipboard": {
        "all": true
      }
    },
    "bundle": {
      "active": true,
      "targets": "all",
      "identifier": "com.scypher.app",
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/128x128@2x.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ]
    },
    "security": {
      "csp": null
    },
    "updater": {
      "active": false
    },
    "windows": [
      {
        "fullscreen": false,
        "height": 800,
        "resizable": true,
        "title": "SCypher",
        "width": 1000,
        "center": true,
        "minHeight": 600,
        "minWidth": 800
      }
    ]
  }
}
```

### Step 1.4: Create Tauri Commands

**File: `src-tauri/src/commands.rs`**

```rust
use tauri::command;
use serde::{Deserialize, Serialize};
use crate::error::{SCypherError, Result};

#[derive(Serialize, Deserialize)]
pub struct SeedValidation {
    pub valid: bool,
    pub word_count: usize,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct AddressSet {
    pub bitcoin: Vec<Address>,
    pub ethereum: Vec<Address>,
    pub ergo: Vec<Address>,
}

#[derive(Serialize, Deserialize)]
pub struct Address {
    pub address_type: String,
    pub path: String,
    pub address: String,
}

#[command]
pub fn validate_seed_phrase(phrase: String) -> Result<SeedValidation> {
    match crate::bip39::validate_seed_phrase_complete(&phrase) {
        Ok(()) => Ok(SeedValidation {
            valid: true,
            word_count: phrase.split_whitespace().count(),
            message: "Valid BIP39 seed phrase".to_string(),
        }),
        Err(e) => Ok(SeedValidation {
            valid: false,
            word_count: phrase.split_whitespace().count(),
            message: e.to_string(),
        }),
    }
}

#[command]
pub fn transform_seed_phrase(
    phrase: String,
    password: String,
    iterations: u32,
    memory_cost: u32,
) -> Result<String> {
    crate::transform_seed(&phrase, &password, iterations, memory_cost)
}

#[command]
pub fn get_bip39_wordlist() -> Vec<String> {
    crate::bip39::BIP39_WORDLIST.iter().map(|s| s.to_string()).collect()
}

#[command]
pub fn validate_bip39_word(word: String) -> bool {
    crate::bip39::is_valid_word(&word)
}

#[command]
pub fn generate_seed_phrase(word_count: usize) -> Result<String> {
    // Implementation will be added in Phase 4
    todo!("Generate seed phrase - implement in Phase 4")
}

#[command]
pub fn derive_addresses(
    seed_phrase: String,
    passphrase: Option<String>,
    networks: Vec<String>,
) -> Result<AddressSet> {
    // Implementation will be added in Phase 4
    todo!("Address derivation - implement in Phase 4")
}
```

### Step 1.5: Update main.rs

**File: `src-tauri/src/main.rs`**

```rust
// Prevent console window on Windows release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

// Import your existing modules
mod crypto;
mod bip39;
mod security;
mod error;

pub use error::{SCypherError, Result};
pub use crypto::transform_seed;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::validate_seed_phrase,
            commands::transform_seed_phrase,
            commands::get_bip39_wordlist,
            commands::validate_bip39_word,
            commands::generate_seed_phrase,
            commands::derive_addresses,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Step 1.6: Test Phase 1

```bash
# Test compilation
cd src-tauri
cargo check

# Test Tauri setup
cargo tauri dev

# Expected: Empty window opens, no errors in console
# Create simple test frontend to verify commands work
```

**Test Frontend: `src/index.html`**

```html
<!DOCTYPE html>
<html>
<head>
    <title>SCypher Test</title>
</head>
<body>
    <h1>SCypher GUI Test</h1>
    <button onclick="testCommand()">Test Backend Connection</button>
    <div id="result"></div>

    <script>
        const { invoke } = window.__TAURI__.tauri;
        
        async function testCommand() {
            try {
                const result = await invoke('validate_bip39_word', { word: 'abandon' });
                document.getElementById('result').innerHTML = `Backend working: ${result}`;
            } catch (error) {
                document.getElementById('result').innerHTML = `Error: ${error}`;
            }
        }
    </script>
</body>
</html>
```

**✅ Phase 1 Complete When:**
- Tauri window opens without errors
- Test command returns expected result
- No compilation errors

---

## 🎨 PHASE 2: Frontend Core Implementation (2-3 days)

### Step 2.1: Create Main HTML Structure

**File: `src/index.html`**

Copy the complete HTML structure from the provided maqueta v12, ensuring all elements have proper IDs.

### Step 2.2: Implement CSS Styling

**File: `src/styles.css`**

Copy the complete CSS from maqueta v12. No modifications needed initially.

### Step 2.3: Core JavaScript Implementation

**File: `src/main.js`**

```javascript
// SCypher GUI Main JavaScript
const { invoke } = window.__TAURI__.tauri;

// Global state
let currentWords = [];
let currentMode = 'auto';
let editingIndex = -1;
let highlightedIndex = -1;
let secureMode = false;
let bip39WordList = [];

// Initialize application
document.addEventListener('DOMContentLoaded', async function() {
    try {
        // Load BIP39 wordlist from backend
        bip39WordList = await invoke('get_bip39_wordlist');
        console.log(`Loaded ${bip39WordList.length} BIP39 words`);
        
        // Initialize event listeners
        initializeEventListeners();
        updateValidationStatus();
        updateProcessButtonState();
    } catch (error) {
        console.error('Failed to initialize:', error);
        alert('Failed to initialize application. Please restart.');
    }
});

// Event listener initialization (copy from maqueta v12)
function initializeEventListeners() {
    // Implementation from maqueta v12
    // ... (copy complete implementation)
}

// Word input handling (copy from maqueta v12)
function handleWordInput(e) {
    // Implementation from maqueta v12
    // ... (copy complete implementation)
}

// Update BIP39 validation to use backend
async function validateSingleWord(word) {
    try {
        return await invoke('validate_bip39_word', { word: word.toLowerCase() });
    } catch (error) {
        console.error('Word validation error:', error);
        return false;
    }
}

// Update seed phrase validation to use backend
async function validateCurrentPhrase() {
    if (currentWords.length === 0) {
        return { valid: false, status: 'empty', message: 'Ready to input seed phrase • AUTO mode active' };
    }

    const phrase = currentWords.join(' ');
    
    try {
        const validation = await invoke('validate_seed_phrase', { phrase });
        
        if (validation.valid) {
            return {
                valid: true,
                status: 'valid',
                message: `✅ Valid BIP39 seed phrase (${validation.word_count} words) with correct checksum`
            };
        } else {
            return {
                valid: false,
                status: 'invalid',
                message: validation.message
            };
        }
    } catch (error) {
        return {
            valid: false,
            status: 'invalid',
            message: `Validation error: ${error}`
        };
    }
}

// Update process function to use backend
async function processSeed() {
    const phrase = currentWords.join(' ');
    const password = document.getElementById('passwordInput').value;
    
    // UI updates for processing state
    const progressBar = document.getElementById('progressBar');
    const progressFill = document.getElementById('progressFill');
    const resultArea = document.getElementById('resultArea');
    const statusText = document.getElementById('statusText');
    const processButton = document.getElementById('processButton');
    
    processButton.disabled = true;
    processButton.textContent = '⏳ Processing...';
    
    progressBar.classList.add('show');
    progressFill.style.width = '0%';
    
    try {
        // Simulate processing stages
        statusText.textContent = 'Converting seed to binary representation...';
        progressFill.style.width = '20%';
        
        await new Promise(resolve => setTimeout(resolve, 300));
        
        statusText.textContent = 'Deriving keystream with Argon2id...';
        progressFill.style.width = '60%';
        
        // Call backend
        const result = await invoke('transform_seed_phrase', {
            phrase,
            password,
            iterations: 5,
            memoryCost: 131072
        });
        
        progressFill.style.width = '85%';
        statusText.textContent = 'Finalizing transformation...';
        
        await new Promise(resolve => setTimeout(resolve, 500));
        
        progressFill.style.width = '100%';
        
        // Show result
        document.getElementById('resultText').textContent = result;
        
        await new Promise(resolve => setTimeout(resolve, 300));
        
        progressBar.classList.remove('show');
        resultArea.classList.add('show');
        
        statusText.textContent = `Ready • Transformation completed • ${currentWords.length} words processed`;
        processButton.disabled = false;
        processButton.textContent = '🔄 Process Another';
        
    } catch (error) {
        console.error('Processing error:', error);
        alert(`Processing failed: ${error}`);
        
        // Reset UI state
        progressBar.classList.remove('show');
        processButton.disabled = false;
        processButton.textContent = '🔄 Process Seed Phrase';
        statusText.textContent = 'Ready • Error occurred during processing';
    }
}

// Copy all other functions from maqueta v12
// ... (copy complete implementation)
```

### Step 2.4: Test Phase 2

```bash
# Test frontend
cargo tauri dev

# Test scenarios:
# 1. Word input and autocompletion
# 2. Seed phrase validation
# 3. Complete processing flow
# 4. UI responsiveness
```

**✅ Phase 2 Complete When:**
- All UI elements render correctly
- Word input system works with real BIP39 validation
- Seed processing works end-to-end
- No JavaScript errors in console

---

## 🔧 PHASE 3: Advanced Features Implementation (2-3 days)

### Step 3.1: File Operations

**Update: `src-tauri/src/commands.rs`**

```rust
use tauri::api::dialog;
use std::path::PathBuf;

#[command]
pub async fn read_seed_file(path: String) -> Result<String> {
    use std::fs;
    match fs::read_to_string(&path) {
        Ok(content) => {
            let cleaned = content
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .collect::<Vec<&str>>()
                .join(" ");
            Ok(cleaned)
        }
        Err(e) => Err(SCypherError::file(format!("Cannot read file: {}", e))),
    }
}

#[command]
pub async fn save_result_file(content: String, path: String) -> Result<()> {
    use std::fs;
    fs::write(&path, &content)
        .map_err(|e| SCypherError::file(format!("Cannot write file: {}", e)))?;
    
    // Set secure permissions on Unix systems
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&path)?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(&path, perms)?;
    }
    
    Ok(())
}

#[command]
pub async fn open_file_dialog() -> Result<Option<String>> {
    use tauri::api::dialog::FileDialogBuilder;
    
    let file_path = FileDialogBuilder::new()
        .add_filter("Text files", &["txt"])
        .pick_file()
        .await;
    
    Ok(file_path.map(|p| p.to_string_lossy().to_string()))
}

#[command]
pub async fn save_file_dialog() -> Result<Option<String>> {
    use tauri::api::dialog::FileDialogBuilder;
    
    let file_path = FileDialogBuilder::new()
        .add_filter("Text files", &["txt"])
        .set_file_name("scypher_result.txt")
        .save_file()
        .await;
    
    Ok(file_path.map(|p| p.to_string_lossy().to_string()))
}
```

### Step 3.2: Seed Generation

**Update: `src-tauri/src/commands.rs`**

```rust
#[command]
pub fn generate_seed_phrase(word_count: usize) -> Result<String> {
    use rand::Rng;
    
    // Validate word count
    let valid_counts = [12, 15, 18, 21, 24];
    if !valid_counts.contains(&word_count) {
        return Err(SCypherError::InvalidWordCount(word_count));
    }
    
    let entropy_bits = word_count * 32 / 3;
    let entropy_bytes = entropy_bits / 8;
    
    // Generate random entropy
    let mut entropy = vec![0u8; entropy_bytes];
    rand::thread_rng().fill(&mut entropy[..]);
    
    // Convert to valid BIP39 phrase
    crate::bip39::conversion::entropy_to_phrase(&entropy)
}
```

### Step 3.3: Enhanced Frontend Integration

**Update: `src/main.js`**

```javascript
// File operations
async function handleFileSelect(e) {
    const file = e.target.files[0];
    if (file) {
        try {
            const content = await invoke('read_seed_file', { path: file.path });
            const words = content.split(/\s+/).filter(word => word.length > 0);
            
            currentWords = words.map(word => word.toLowerCase());
            editingIndex = -1;
            renderWords();
            updateValidationStatus();
            updateProcessButtonState();
        } catch (error) {
            alert(`Failed to read file: ${error}`);
        }
    }
}

// Enhanced drag and drop
async function handleDrop(e) {
    e.preventDefault();
    e.currentTarget.classList.remove('drag-over');
    
    const files = e.dataTransfer.files;
    if (files.length > 0) {
        try {
            const content = await invoke('read_seed_file', { path: files[0].path });
            const words = content.split(/\s+/).filter(word => word.length > 0);
            
            currentWords = words.map(word => word.toLowerCase());
            editingIndex = -1;
            renderWords();
            updateValidationStatus();
            updateProcessButtonState();
        } catch (error) {
            alert(`Failed to read file: ${error}`);
        }
    }
}

// File save functionality
async function saveToFile() {
    try {
        const filePath = await invoke('save_file_dialog');
        if (filePath) {
            const resultText = document.getElementById('resultText').textContent;
            await invoke('save_result_file', { content: resultText, path: filePath });
            alert('File saved successfully!');
        }
    } catch (error) {
        alert(`Failed to save file: ${error}`);
    }
}

// Generate seed functionality
async function generateNewSeed() {
    try {
        const targetCount = currentMode === 'auto' ? 12 : parseInt(currentMode);
        const generatedPhrase = await invoke('generate_seed_phrase', { wordCount: targetCount });
        
        currentWords = generatedPhrase.split(' ');
        editingIndex = -1;
        renderWords();
        updateValidationStatus();
        updateProcessButtonState();
        focusWordInput();
    } catch (error) {
        alert(`Failed to generate seed: ${error}`);
    }
}

// Update copy to clipboard
async function copyToClipboard(text) {
    try {
        await navigator.clipboard.writeText(text);
        return true;
    } catch (error) {
        console.error('Clipboard error:', error);
        return false;
    }
}
```

### Step 3.4: Test Phase 3

```bash
# Test all new features
cargo tauri dev

# Test scenarios:
# 1. File drag and drop
# 2. File browser dialog
# 3. Save file dialog
# 4. Seed generation
# 5. Clipboard operations
```

**✅ Phase 3 Complete When:**
- File operations work correctly
- Seed generation produces valid BIP39 phrases
- Clipboard functionality works
- All dialogs open and function properly

---

## 🏠 PHASE 4: Address Derivation System (2-3 days)

### Step 4.1: HD Wallet Dependencies

**Update: `src-tauri/Cargo.toml`**

```toml
# Add HD wallet dependencies
bip32 = "0.5"
bip39 = "2.0"
bitcoin = "0.30"
secp256k1 = "0.27"
ethereum-types = "0.14"
tiny-keccak = "2.0"

# For Ergo addresses (if available)
# ergo-lib = "0.4"  # Check for latest version
```

### Step 4.2: Address Derivation Implementation

**Create: `src-tauri/src/addresses.rs`**

```rust
use serde::{Deserialize, Serialize};
use crate::error::{SCypherError, Result};

#[derive(Serialize, Deserialize, Clone)]
pub struct Address {
    pub address_type: String,
    pub path: String,
    pub address: String,
}

#[derive(Serialize, Deserialize)]
pub struct AddressSet {
    pub bitcoin: Vec<Address>,
    pub ethereum: Vec<Address>,
    pub ergo: Vec<Address>,
}

pub fn derive_addresses(
    seed_phrase: &str,
    passphrase: Option<&str>,
    networks: &[String],
) -> Result<AddressSet> {
    use bip39::{Mnemonic, Seed};
    use bip32::{XPrv, DerivationPath};
    use std::str::FromStr;
    
    // Parse mnemonic
    let mnemonic = Mnemonic::from_phrase(seed_phrase, bip39::Language::English)
        .map_err(|e| SCypherError::crypto(format!("Invalid mnemonic: {}", e)))?;
    
    // Generate seed with optional passphrase
    let seed = Seed::new(&mnemonic, passphrase.unwrap_or(""));
    
    // Derive master key
    let master_key = XPrv::new(seed.as_bytes())
        .map_err(|e| SCypherError::crypto(format!("Key derivation failed: {}", e)))?;
    
    let mut address_set = AddressSet {
        bitcoin: Vec::new(),
        ethereum: Vec::new(),
        ergo: Vec::new(),
    };
    
    for network in networks {
        match network.as_str() {
            "bitcoin" => {
                address_set.bitcoin = derive_bitcoin_addresses(&master_key)?;
            }
            "ethereum" => {
                address_set.ethereum = derive_ethereum_addresses(&master_key)?;
            }
            "ergo" => {
                address_set.ergo = derive_ergo_addresses(&master_key)?;
            }
            _ => return Err(SCypherError::crypto(format!("Unsupported network: {}", network))),
        }
    }
    
    Ok(address_set)
}

fn derive_bitcoin_addresses(master_key: &XPrv) -> Result<Vec<Address>> {
    use bip32::DerivationPath;
    use bitcoin::{Address as BtcAddress, Network, PublicKey};
    use std::str::FromStr;
    
    let mut addresses = Vec::new();
    
    // P2PKH (Legacy) - m/44'/0'/0'/0/0
    let path = DerivationPath::from_str("m/44'/0'/0'/0/0").unwrap();
    let child_key = master_key.derive_path(&path)
        .map_err(|e| SCypherError::crypto(format!("Bitcoin derivation failed: {}", e)))?;
    
    let public_key = PublicKey::from_private_key(
        &bitcoin::secp256k1::Secp256k1::new(),
        &child_key.private_key().into()
    );
    
    let address = BtcAddress::p2pkh(&public_key, Network::Bitcoin);
    addresses.push(Address {
        address_type: "Legacy (P2PKH)".to_string(),
        path: "m/44'/0'/0'/0/0".to_string(),
        address: address.to_string(),
    });
    
    // Add more Bitcoin address types (P2WPKH, P2SH-P2WPKH)
    // ... (implement additional types)
    
    Ok(addresses)
}

fn derive_ethereum_addresses(master_key: &XPrv) -> Result<Vec<Address>> {
    use bip32::DerivationPath;
    use tiny_keccak::{Hasher, Keccak};
    use std::str::FromStr;
    
    let mut addresses = Vec::new();
    
    // Ethereum standard - m/44'/60'/0'/0/0
    let path = DerivationPath::from_str("m/44'/60'/0'/0/0").unwrap();
    let child_key = master_key.derive_path(&path)
        .map_err(|e| SCypherError::crypto(format!("Ethereum derivation failed: {}", e)))?;
    
    // Get public key
    let public_key = child_key.public_key();
    let public_key_bytes = public_key.public_key().serialize_uncompressed();
    
    // Hash with Keccak256
    let mut hasher = Keccak::v256();
    hasher.update(&public_key_bytes[1..]); // Skip the 0x04 prefix
    let mut hash = [0u8; 32];
    hasher.finalize(&mut hash);
    
    // Take last 20 bytes as address
    let address_bytes = &hash[12..];
    let address = format!("0x{}", hex::encode(address_bytes));
    
    addresses.push(Address {
        address_type: "Ethereum (Standard)".to_string(),
        path: "m/44'/60'/0'/0/0".to_string(),
        address,
    });
    
    Ok(addresses)
}

fn derive_ergo_addresses(master_key: &XPrv) -> Result<Vec<Address>> {
    // Placeholder for Ergo address derivation
    // Implementation depends on available Ergo libraries
    let mut addresses = Vec::new();
    
    addresses.push(Address {
        address_type: "Ergo (P2PK)".to_string(),
        path: "m/44'/429'/0'/0/0".to_string(),
        address: "9fRAWhdxEsTcdb8PhGNrpfchZnFDQFbpn1vhJN7sX7GZF2s2".to_string(), // Placeholder
    });
    
    Ok(addresses)
}
```

### Step 4.3: Update Commands

**Update: `src-tauri/src/commands.rs`**

```rust
use crate::addresses::{derive_addresses as derive_addr, AddressSet};

#[command]
pub fn derive_addresses(
    seed_phrase: String,
    passphrase: Option<String>,
    networks: Vec<String>,
) -> Result<AddressSet> {
    derive_addr(&seed_phrase, passphrase.as_deref(), &networks)
}
```

### Step 4.4: Frontend Address Derivation

**Update: `src/main.js`**

```javascript
// Address derivation functions
async function deriveAddresses() {
    const selectedNetworks = Array.from(document.querySelectorAll('.network-button.active'))
        .map(btn => btn.dataset.network);
    
    if (selectedNetworks.length === 0) return;

    const seedPhrase = document.getElementById('resultText').textContent;
    const passphrase = document.getElementById('bip39Passphrase').value;
    
    const addressesSection = document.getElementById('addressesSection');
    const deriveButton = document.getElementById('deriveAddresses');
    const refreshButton = document.getElementById('refreshAddresses');
    
    deriveButton.disabled = true;
    deriveButton.textContent = '⏳ Deriving...';
    
    addressesSection.style.display = 'block';
    showAddressLoading();
    
    try {
        const addresses = await invoke('derive_addresses', {
            seedPhrase,
            passphrase: passphrase.length > 0 ? passphrase : null,
            networks: selectedNetworks
        });
        
        generateAddressDisplay(selectedNetworks, addresses, passphrase.length > 0);
        
        deriveButton.disabled = false;
        deriveButton.textContent = '✅ Addresses Derived';
        refreshButton.disabled = false;
        refreshButton.textContent = '🔄 Refresh';
        
    } catch (error) {
        console.error('Address derivation error:', error);
        alert(`Failed to derive addresses: ${error}`);
        
        deriveButton.disabled = false;
        deriveButton.textContent = `🏠 Derive ${selectedNetworks.length} Network${selectedNetworks.length > 1 ? 's' : ''}`;
    }
}

function generateAddressDisplay(networks, addresses, hasPassphrase) {
    const addressContent = document.getElementById('addressContent');
    const passphraseIndicator = document.getElementById('passphraseIndicator');
    
    if (hasPassphrase) {
        passphraseIndicator.style.display = 'flex';
    } else {
        passphraseIndicator.style.display = 'none';
    }
    
    let html = '';
    
    // Create network tabs
    html += '<div class="address-network-tabs">';
    networks.forEach((network, index) => {
        const icons = { bitcoin: '₿', ethereum: 'Ξ', ergo: '⚡' };
        const names = { bitcoin: 'Bitcoin', ethereum: 'Ethereum', ergo: 'Ergo' };
        html += `<button class="network-tab ${index === 0 ? 'active' : ''}" data-network="${network}">
            ${icons[network]} ${names[network]}
        </button>`;
    });
    html += '</div>';
    
    // Create network content
    networks.forEach((network, index) => {
        const networkAddresses = addresses[network] || [];
        
        html += `<div class="address-network ${index === 0 ? 'active' : ''}" data-network="${network}">
            <div class="address-grid">`;
        
        networkAddresses.forEach(addr => {
            html += `
                <div class="address-card">
                    <div class="address-type">
                        <span>${addr.address_type}</span>
                        <div class="address-actions">
                            <button class="copy-button mini" onclick="copyAddressToClipboard('${addr.address}')">📋</button>
                            <button class="copy-button mini" onclick="showQR('${addr.address}')">📱</button>
                        </div>
                    </div>
                    <div class="address-path">${addr.path}</div>
                    <div class="address-value">${addr.address}</div>
                </div>`;
        });
        
        html += '</div></div>';
    });
    
    addressContent.innerHTML = html;
    
    // Re-attach event listeners for new tabs
    document.querySelectorAll('.network-tab').forEach(tab => {
        tab.addEventListener('click', (e) => switchNetworkTab(e.target.dataset.network));
    });
}

async function copyAddressToClipboard(address) {
    try {
        await navigator.clipboard.writeText(address);
        console.log('Address copied to clipboard');
    } catch (error) {
        console.error('Failed to copy address:', error);
    }
}

// Information modal functionality
function showInfoModal() {
    const modal = document.createElement('div');
    modal.style.cssText = `
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: rgba(0, 0, 0, 0.8);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 1000;
    `;
    
    modal.innerHTML = `
        <div style="
            background: rgba(40, 40, 40, 0.95);
            border: 1px solid #4a4a4a;
            border-radius: 12px;
            padding: 30px;
            max-width: 500px;
            margin: 20px;
            color: #e0e0e0;
            font-family: 'Inter', sans-serif;
        ">
            <h2 style="color: #ff9500; margin-bottom: 20px; text-align: center;">🔒 How SCypher Security Works</h2>
            
            <p style="margin-bottom: 15px;">You have <strong>TWO separate passwords:</strong></p>
            
            <div style="margin-bottom: 20px;">
                <h3 style="color: #ff9500; margin-bottom: 8px;">🔐 SCypher Password:</h3>
                <ul style="margin-left: 20px; line-height: 1.6;">
                    <li>Transforms your master seed into a different but normal-looking seed</li>
                    <li>The new seed looks completely normal</li>
                    <li>Same password transforms it back to original</li>
                </ul>
            </div>
            
            <div style="margin-bottom: 20px;">
                <h3 style="color: #ff9500; margin-bottom: 8px;">🔑 BIP39 Passphrase (optional):</h3>
                <ul style="margin-left: 20px; line-height: 1.6;">
                    <li>Acts like an extra word added to your seed</li>
                    <li>Applied to the SCypher-transformed seed</li>
                    <li>Changes which addresses are generated</li>
                </ul>
            </div>
            
            <div style="margin-bottom: 20px;">
                <h3 style="color: #ff9500; margin-bottom: 8px;">📝 To get back to your master seed:</h3>
                <ol style="margin-left: 20px; line-height: 1.6;">
                    <li>Take your transformed seed</li>
                    <li>Use same SCypher Password → Get master seed</li>
                    <li>Use BIP39 Passphrase → Access specific wallet</li>
                </ol>
            </div>
            
            <div style="background: rgba(255, 193, 7, 0.1); border: 1px solid rgba(255, 193, 7, 0.3); border-radius: 6px; padding: 15px; margin-bottom: 20px;">
                <h3 style="color: #ffc107; margin-bottom: 8px;">⚠️ REMEMBER:</h3>
                <ul style="margin-left: 20px; line-height: 1.6; color: #ffc107;">
                    <li>You need BOTH to access your money</li>
                    <li>Write them down separately</li>
                    <li>Don't store them together</li>
                </ul>
            </div>
            
            <div style="text-align: center;">
                <button onclick="this.parentElement.parentElement.parentElement.remove()" style="
                    background: linear-gradient(135deg, #ff9500 0%, #ff7b00 100%);
                    color: white;
                    border: none;
                    padding: 12px 24px;
                    border-radius: 6px;
                    font-weight: 600;
                    cursor: pointer;
                ">✓ Got it!</button>
            </div>
        </div>
    `;
    
    document.body.appendChild(modal);
    
    // Close on outside click
    modal.addEventListener('click', (e) => {
        if (e.target === modal) {
            modal.remove();
        }
    });
}
```

### Step 4.5: Update main.rs

**Update: `src-tauri/src/main.rs`**

```rust
mod commands;
mod addresses;  // Add this line

// Import your existing modules
mod crypto;
mod bip39;
mod security;
mod error;

pub use error::{SCypherError, Result};
pub use crypto::transform_seed;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::validate_seed_phrase,
            commands::transform_seed_phrase,
            commands::get_bip39_wordlist,
            commands::validate_bip39_word,
            commands::generate_seed_phrase,
            commands::derive_addresses,
            commands::read_seed_file,
            commands::save_result_file,
            commands::open_file_dialog,
            commands::save_file_dialog,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Step 4.6: Add Info Button to HTML

**Update: `src/index.html`** (in the passphrase section)

```html
<div class="input-group">
    <label class="label">
        BIP39 Passphrase (13th word):
        <button onclick="showInfoModal()" style="
            background: rgba(60, 60, 60, 0.8);
            border: 1px solid #555;
            color: #ccc;
            padding: 2px 6px;
            border-radius: 4px;
            cursor: pointer;
            font-size: 0.7em;
            margin-left: 8px;
        ">ℹ️ What's this?</button>
    </label>
    <!-- rest of passphrase input -->
</div>
```

### Step 4.7: Test Phase 4

```bash
# Test address derivation
cargo tauri dev

# Test scenarios:
# 1. Derive Bitcoin addresses
# 2. Derive Ethereum addresses  
# 3. Derive with and without passphrase
# 4. Info modal display
# 5. Address copying
```

**✅ Phase 4 Complete When:**
- Address derivation works for all networks
- Passphrase affects address generation
- Info modal explains the system clearly
- All addresses can be copied to clipboard

---

## 🛡️ PHASE 5: Security and Polish (1-2 days)

### Step 5.1: Enhanced Security Features

**Update: `src-tauri/src/commands.rs`**

```rust
use zeroize::Zeroize;

#[command]
pub fn secure_clear_memory() -> Result<()> {
    // Force garbage collection if possible
    std::hint::black_box(());
    Ok(())
}

// Update existing commands to use zeroize
#[command]
pub fn transform_seed_phrase(
    mut phrase: String,
    mut password: String,
    iterations: u32,
    memory_cost: u32,
) -> Result<String> {
    let result = crate::transform_seed(&phrase, &password, iterations, memory_cost);
    
    // Securely clear sensitive data
    phrase.zeroize();
    password.zeroize();
    
    result
}
```

### Step 5.2: Error Handling Enhancement

**Update: `src/main.js`**

```javascript
// Global error handler
window.addEventListener('error', (event) => {
    console.error('Application error:', event.error);
    // Don't show sensitive data in error messages
    if (event.error.message.includes(sensitiveTerms)) {
        event.preventDefault();
    }
});

// Enhanced error display
function showError(message, title = 'Error') {
    const modal = document.createElement('div');
    modal.style.cssText = `
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: rgba(0, 0, 0, 0.8);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 1000;
    `;
    
    modal.innerHTML = `
        <div style="
            background: rgba(40, 40, 40, 0.95);
            border: 1px solid #ef4444;
            border-radius: 12px;
            padding: 30px;
            max-width: 400px;
            margin: 20px;
            color: #e0e0e0;
            text-align: center;
        ">
            <h2 style="color: #ef4444; margin-bottom: 15px;">❌ ${title}</h2>
            <p style="margin-bottom: 20px; line-height: 1.6;">${message}</p>
            <button onclick="this.parentElement.parentElement.remove()" style="
                background: #ef4444;
                color: white;
                border: none;
                padding: 10px 20px;
                border-radius: 6px;
                cursor: pointer;
            ">OK</button>
        </div>
    `;
    
    document.body.appendChild(modal);
}

// Secure mode enhancements
function toggleSecureMode() {
    secureMode = !secureMode;
    const button = document.getElementById('secureMode');
    const mainContent = document.getElementById('mainContent');
    const securityStatus = document.getElementById('securityStatus');
    
    if (secureMode) {
        button.textContent = '🛡️ Secure Screen: ON';
        button.classList.add('active');
        mainContent.classList.add('secure-hidden');
        securityStatus.textContent = 'Secure mode active - content hidden';
        
        // Clear clipboard if in secure mode
        if (navigator.clipboard) {
            navigator.clipboard.writeText('').catch(() => {});
        }
    } else {
        button.textContent = '🛡️ Secure Screen: OFF';
        button.classList.remove('active');
        mainContent.classList.remove('secure-hidden');
        securityStatus.textContent = 'Secure mode ready';
    }
}

// Memory cleanup on page unload
window.addEventListener('beforeunload', () => {
    // Clear sensitive variables
    currentWords = [];
    bip39WordList = [];
    
    // Clear input fields
    document.getElementById('wordInput').value = '';
    document.getElementById('passwordInput').value = '';
    document.getElementById('bip39Passphrase').value = '';
    
    // Invoke backend cleanup
    invoke('secure_clear_memory').catch(() => {});
});
```

### Step 5.3: Performance Optimizations

**Update: `src/main.js`**

```javascript
// Debounce word input for better performance
function debounce(func, wait) {
    let timeout;
    return function executedFunction(...args) {
        const later = () => {
            clearTimeout(timeout);
            func(...args);
        };
        clearTimeout(timeout);
        timeout = setTimeout(later, wait);
    };
}

// Debounced validation
const debouncedValidation = debounce(async function() {
    const validation = await validateCurrentPhrase();
    updateValidationDisplay(validation);
}, 300);

// Use debounced validation in word input
function handleWordInput(e) {
    // ... existing logic ...
    
    // Use debounced validation instead of immediate
    debouncedValidation();
}

// Optimize autocomplete rendering
function showAutocomplete(query) {
    const dropdown = document.getElementById('autocompleteDropdown');
    
    // Use efficient filtering
    const matches = bip39WordList.filter(word => 
        word.startsWith(query.toLowerCase())
    ).slice(0, 8);

    if (matches.length === 0) {
        hideAutocomplete();
        return;
    }

    // Use document fragment for better performance
    const fragment = document.createDocumentFragment();
    matches.forEach((word, index) => {
        const item = document.createElement('div');
        item.className = 'autocomplete-item';
        item.dataset.word = word;
        item.dataset.index = index;
        item.innerHTML = `
            <span class="word-text">${word}</span>
            <span class="word-hint">${word.length} chars</span>
        `;
        fragment.appendChild(item);
    });

    dropdown.innerHTML = '';
    dropdown.appendChild(fragment);
    highlightedIndex = -1;
    dropdown.classList.add('show');
}
```

### Step 5.4: Final Testing and Bug Fixes

```bash
# Comprehensive testing
cargo tauri dev

# Test all features end-to-end:
# 1. Word input system
# 2. File operations
# 3. Seed processing
# 4. Address derivation
# 5. Security features
# 6. Error handling
# 7. Performance under load

# Test edge cases:
# - Very long passwords
# - Invalid file formats
# - Network errors
# - Memory constraints
```

### Step 5.5: Build for Production

```bash
# Build optimized version
cargo tauri build

# Test the built application
# Verify file size and performance
# Test on different operating systems if possible
```

**✅ Phase 5 Complete When:**
- All security features work correctly
- Error handling is robust
- Performance is optimized
- Production build works without issues
- All edge cases are handled

---

## 🚀 DEPLOYMENT AND FINAL STEPS

### Step 6.1: Application Icons

Create application icons in the required sizes:
- `src-tauri/icons/32x32.png`
- `src-tauri/icons/128x128.png`
- `src-tauri/icons/128x128@2x.png`
- `src-tauri/icons/icon.icns` (macOS)
- `src-tauri/icons/icon.ico` (Windows)

### Step 6.2: Final Configuration

**Update: `src-tauri/tauri.conf.json`**

```json
{
  "package": {
    "productName": "SCypher",
    "version": "3.0.0"
  },
  "tauri": {
    "bundle": {
      "active": true,
      "targets": "all",
      "identifier": "com.scypher.app",
      "publisher": "SCypher Team",
      "copyright": "Copyright © 2024 SCypher Team",
      "category": "DeveloperTool",
      "shortDescription": "XOR-based BIP39 Seed Cipher",
      "longDescription": "Professional tool for secure BIP39 seed phrase transformation using XOR encryption with Argon2id key derivation."
    }
  }
}
```

### Step 6.3: Documentation

Create comprehensive documentation:
- User manual
- Security best practices
- Troubleshooting guide
- Development setup instructions

### Step 6.4: Testing Matrix

| Feature | Windows | macOS | Linux |
|---------|---------|-------|-------|
| Word input system | ✅ | ✅ | ✅ |
| File operations | ✅ | ✅ | ✅ |
| Seed processing | ✅ | ✅ | ✅ |
| Address derivation | ✅ | ✅ | ✅ |
| Security features | ✅ | ✅ | ✅ |

---

## 🔧 TROUBLESHOOTING GUIDE

### Common Issues and Solutions

**Issue: Tauri compilation fails**
```bash
# Solution: Update Rust and dependencies
rustup update
cargo update
```

**Issue: Frontend can't communicate with backend**
```bash
# Solution: Check command registration in main.rs
# Verify function names match between frontend and backend
```

**Issue: Address derivation fails**
```bash
# Solution: Check HD wallet dependencies
# Verify BIP39 mnemonic validity
# Ensure correct derivation paths
```

**Issue: File operations don't work**
```bash
# Solution: Check Tauri allowlist permissions
# Verify file paths are correct
# Check file permissions
```

### Performance Optimization Tips

1. **Memory Usage**: Monitor memory consumption during long operations
2. **UI Responsiveness**: Use debounced operations for real-time validation
3. **Build Size**: Optimize dependencies and remove unused features
4. **Startup Time**: Minimize initialization overhead

### Security Checklist

- [ ] No sensitive data in console logs
- [ ] Memory cleared after operations
- [ ] Secure file permissions set
- [ ] No sensitive data in error messages
- [ ] Clipboard cleared in secure mode
- [ ] Network requests disabled
- [ ] Debug information removed from production builds

---

## 📚 ADDITIONAL RESOURCES

### Tauri Documentation
- [Tauri Guides](https://tauri.app/v1/guides/)
- [API Reference](https://tauri.app/v1/api/)
- [Configuration](https://tauri.app/v1/api/config/)

### BIP39 and HD Wallets
- [BIP39 Specification](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki)
- [BIP32 HD Wallets](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki)
- [BIP44 Multi-Account Hierarchy](https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki)

### Cryptography References
- [Argon2 Specification](https://tools.ietf.org/html/rfc9106)
- [XOR Cipher Security](https://en.wikipedia.org/wiki/XOR_cipher)

---

## ✅ COMPLETION CRITERIA

### Phase Completion Checkmarks

**Phase 1: Tauri Setup**
- [ ] Tauri project initialized
- [ ] Backend commands registered
- [ ] Basic communication working
- [ ] Test frontend loads

**Phase 2: Frontend Core**
- [ ] Complete UI implemented
- [ ] Word input system working
- [ ] BIP39 validation functional
- [ ] Seed processing end-to-end

**Phase 3: Advanced Features**
- [ ] File operations working
- [ ] Seed generation functional
- [ ] Clipboard operations working
- [ ] All dialogs functional

**Phase 4: Address Derivation**
- [ ] HD wallet implementation complete
- [ ] All networks supported
- [ ] Passphrase system working
- [ ] Info modal implemented

**Phase 5: Security and Polish**
- [ ] Security features implemented
- [ ] Error handling robust
- [ ] Performance optimized
- [ ] Production build working

**Final Deployment**
- [ ] Icons created
- [ ] Configuration finalized
- [ ] Documentation complete
- [ ] Cross-platform testing done

---

*This implementation guide is designed to be self-contained and can be used by any AI assistant to help implement SCypher GUI from any phase of the project.*