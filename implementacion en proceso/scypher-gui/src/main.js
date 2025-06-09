// SCypher GUI Main JavaScript - Phase 2 - Fixed for Tauri

// Global state
let currentWords = [];
let currentMode = 'auto';
let editingIndex = -1;
let highlightedIndex = -1;
let secureMode = false;
let bip39WordList = [];

// Helper function to get Tauri API safely
function getTauriAPI() {
    if (window.__TAURI__ && window.__TAURI__.tauri) {
        return window.__TAURI__.tauri;
    }
    throw new Error('Tauri API not available');
}

// Initialize application when DOM loads
document.addEventListener('DOMContentLoaded', async function() {
    // Wait a bit for Tauri to be fully loaded
    await new Promise(resolve => setTimeout(resolve, 100));

    try {
        console.log('Initializing SCypher GUI...');

        // Get Tauri API
        const { invoke } = getTauriAPI();

        // Load BIP39 wordlist from backend
        console.log('Loading BIP39 wordlist...');
        bip39WordList = await invoke('get_bip39_wordlist');
        console.log(`✓ Loaded ${bip39WordList.length} BIP39 words from backend`);

        // Initialize event listeners
        initializeEventListeners();
        updateValidationStatus();
        updateProcessButtonState();

        console.log('✓ SCypher GUI initialized successfully');
    } catch (error) {
        console.error('❌ Failed to initialize application:', error);
        showError('Failed to initialize application. Please check the backend connection.', 'Initialization Error');
    }
});

// Initialize all event listeners
function initializeEventListeners() {
    console.log('Setting up event listeners...');

    // Word input events
    const wordInput = document.getElementById('wordInput');
    wordInput.addEventListener('input', handleWordInput);
    wordInput.addEventListener('keydown', handleKeyDown);
    wordInput.addEventListener('blur', handleInputBlur);

    // Container events
    const container = document.getElementById('wordContainer');
    container.addEventListener('click', focusWordInput);
    container.addEventListener('dragover', handleDragOver);
    container.addEventListener('dragleave', handleDragLeave);
    container.addEventListener('drop', handleDrop);

    // Control events
    document.getElementById('wordCount').addEventListener('change', handleModeChange);
    document.getElementById('generateSeed').addEventListener('click', generateNewSeed);
    document.getElementById('clearInput').addEventListener('click', clearAllWords);
    document.getElementById('browseFile').addEventListener('click', () => document.getElementById('fileInput').click());
    document.getElementById('fileInput').addEventListener('change', handleFileSelect);

    // Security events
    document.getElementById('passwordToggle').addEventListener('click', () =>
        togglePasswordVisibility('passwordInput', document.getElementById('passwordToggle')));
    document.getElementById('passwordInput').addEventListener('input', updateProcessButtonState);
    document.getElementById('secureMode').addEventListener('click', toggleSecureMode);

    // Process button
    document.getElementById('processButton').addEventListener('click', processSeed);

    // Result actions
    document.getElementById('copyResult').addEventListener('click', () => copyResultToClipboard());
    document.getElementById('saveResult').addEventListener('click', () => saveResultToFile());
    document.getElementById('processAnother').addEventListener('click', () => processAnother());

    // Autocomplete events
    document.getElementById('autocompleteDropdown').addEventListener('click', handleAutocompleteClick);

    // Address derivation events (for future phases)
    document.getElementById('passphraseToggle').addEventListener('click', togglePassphraseSection);
    if (document.getElementById('deriveAddresses')) {
        document.getElementById('deriveAddresses').addEventListener('click', deriveAddresses);
    }

    // Network selection events
    document.querySelectorAll('.network-button').forEach(btn => {
        btn.addEventListener('click', (e) => toggleNetworkSelection(e.target));
    });
}

// Handle word input with real-time validation
function handleWordInput(e) {
    const value = e.target.value.trim();

    // Handle multiple words pasted
    if (value.includes(' ')) {
        const words = value.split(/\s+/).filter(word => word.length > 0);
        if (words.length > 1) {
            for (const word of words) {
                if (validateSingleWord(word)) {
                    addWord(word);
                }
            }
            e.target.value = '';
            hideAutocomplete();
            return;
        }
        if (words.length === 1 && validateSingleWord(words[0])) {
            addWord(words[0]);
            e.target.value = '';
            hideAutocomplete();
            return;
        }
    }

    // Show autocomplete for partial words
    if (value.length > 0) {
        showAutocomplete(value);
    } else {
        hideAutocomplete();
    }

    updateContainerState();
}

// Handle keyboard navigation in word input
function handleKeyDown(e) {
    const input = e.target;
    const dropdown = document.getElementById('autocompleteDropdown');
    const items = dropdown.querySelectorAll('.autocomplete-item');

    switch (e.key) {
        case 'Enter':
            e.preventDefault();
            if (highlightedIndex >= 0 && items[highlightedIndex]) {
                selectAutocompleteItem(items[highlightedIndex]);
            } else if (input.value.trim()) {
                const word = input.value.trim();
                if (validateSingleWord(word)) {
                    addWord(word);
                    input.value = '';
                    hideAutocomplete();
                }
            }
            break;

        case 'Tab':
            if (items.length === 1) {
                e.preventDefault();
                selectAutocompleteItem(items[0]);
            }
            break;

        case 'ArrowDown':
            e.preventDefault();
            if (items.length > 0) {
                highlightedIndex = Math.min(highlightedIndex + 1, items.length - 1);
                updateAutocompleteHighlight(items);
            }
            break;

        case 'ArrowUp':
            e.preventDefault();
            if (items.length > 0) {
                highlightedIndex = Math.max(highlightedIndex - 1, 0);
                updateAutocompleteHighlight(items);
            }
            break;

        case 'Escape':
            hideAutocomplete();
            break;

        case 'Backspace':
            if (input.value === '' && currentWords.length > 0) {
                e.preventDefault();
                editLastWord();
            }
            break;
    }
}

function handleInputBlur() {
    setTimeout(() => {
        if (!document.querySelector('.autocomplete-dropdown:hover')) {
            hideAutocomplete();
        }
    }, 150);
}

// Show autocomplete dropdown with BIP39 word suggestions
function showAutocomplete(query) {
    const dropdown = document.getElementById('autocompleteDropdown');
    const matches = bip39WordList.filter(word =>
        word.toLowerCase().startsWith(query.toLowerCase())
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

function hideAutocomplete() {
    const dropdown = document.getElementById('autocompleteDropdown');
    dropdown.classList.remove('show');
    highlightedIndex = -1;
}

function updateAutocompleteHighlight(items) {
    items.forEach((item, index) => {
        item.classList.toggle('highlighted', index === highlightedIndex);
    });
}

function handleAutocompleteClick(e) {
    const item = e.target.closest('.autocomplete-item');
    if (item) {
        selectAutocompleteItem(item);
    }
}

function selectAutocompleteItem(item) {
    const word = item.dataset.word;
    addWord(word);
    document.getElementById('wordInput').value = '';
    hideAutocomplete();
}

// Word management functions
function addWord(word) {
    if (editingIndex >= 0) {
        currentWords[editingIndex] = word.toLowerCase();
        editingIndex = -1;
    } else {
        currentWords.push(word.toLowerCase());
    }
    renderWords();
    updateValidationStatus();
    updateProcessButtonState();
    focusWordInput();
}

function editLastWord() {
    if (currentWords.length > 0) {
        const lastWord = currentWords[currentWords.length - 1];
        currentWords.pop();
        document.getElementById('wordInput').value = lastWord;
        editingIndex = -1;
        renderWords();
        updateValidationStatus();
        updateProcessButtonState();
        focusWordInput();
    }
}

function editWordAtIndex(index) {
    if (index >= 0 && index < currentWords.length) {
        const word = currentWords[index];
        document.getElementById('wordInput').value = word;
        editingIndex = index;
        focusWordInput();
    }
}

function deleteWordAtIndex(index) {
    if (index >= 0 && index < currentWords.length) {
        currentWords.splice(index, 1);
        renderWords();
        updateValidationStatus();
        updateProcessButtonState();
        focusWordInput();
    }
}

function renderWords() {
    const display = document.getElementById('wordsDisplay');
    display.innerHTML = currentWords.map((word, index) => `
        <div class="word-tag ${editingIndex === index ? 'editing' : ''}"
             onclick="editWordAtIndex(${index})"
             data-index="${index}">
            <span class="word-number">${index + 1}</span>
            <span>${word}</span>
            <span class="delete-word" onclick="event.stopPropagation(); deleteWordAtIndex(${index})">×</span>
        </div>
    `).join('');

    updateContainerState();
}

function updateContainerState() {
    const container = document.getElementById('wordContainer');
    const hasContent = currentWords.length > 0 || document.getElementById('wordInput').value.length > 0;
    container.classList.toggle('has-content', hasContent);
}

// Validation functions using backend
function validateSingleWord(word) {
    return bip39WordList.includes(word.toLowerCase());
}

async function validateCurrentPhrase() {
    if (currentWords.length === 0) {
        return {
            valid: false,
            status: 'empty',
            message: 'Ready to input seed phrase • AUTO mode active'
        };
    }

    const phrase = currentWords.join(' ');

    try {
        const { invoke } = getTauriAPI();
        const validation = await invoke('validate_seed_phrase', { phrase });
        return {
            valid: validation.valid,
            status: validation.status,
            message: validation.message
        };
    } catch (error) {
        console.error('Backend validation error:', error);
        return {
            valid: false,
            status: 'invalid',
            message: `Validation error: ${error}`
        };
    }
}

async function updateValidationStatus() {
    const container = document.getElementById('wordContainer');
    const statusElement = document.getElementById('validationStatus');
    const validation = await validateCurrentPhrase();

    container.className = 'word-input-container';
    if (validation.status === 'valid') {
        container.classList.add('valid');
    } else if (validation.status === 'invalid') {
        container.classList.add('invalid');
    }

    const statusClass = validation.status === 'valid' ? 'status-valid' :
                       validation.status === 'invalid' ? 'status-invalid' :
                       validation.status === 'progress' ? 'status-progress' : 'status-progress';

    statusElement.innerHTML = `<span class="${statusClass}">${validation.message}</span>`;

    return validation;
}

async function updateProcessButtonState() {
    const validation = await validateCurrentPhrase();
    const passwordInput = document.getElementById('passwordInput');
    const processButton = document.getElementById('processButton');

    const hasValidSeed = validation.valid;
    const hasPassword = passwordInput.value.length >= 8;

    processButton.disabled = !(hasValidSeed && hasPassword);

    if (hasValidSeed && hasPassword) {
        processButton.textContent = '🔄 Process Seed Phrase';
    } else if (!hasValidSeed) {
        processButton.textContent = '⏳ Complete valid seed phrase...';
    } else {
        processButton.textContent = '⏳ Enter password (min 8 chars)...';
    }
}

// Control functions
function handleModeChange(e) {
    const newMode = e.target.value;
    const selector = e.target;

    if (newMode === 'auto') {
        selector.classList.add('auto');
        currentMode = 'auto';
    } else {
        selector.classList.remove('auto');
        const targetCount = parseInt(newMode);

        if (currentWords.length > targetCount) {
            if (confirm(`You have ${currentWords.length} words but selected ${targetCount}. This will clear extra words. Continue?`)) {
                currentWords = currentWords.slice(0, targetCount);
                renderWords();
            } else {
                e.target.value = currentMode;
                return;
            }
        }

        currentMode = newMode;
    }

    updateValidationStatus();
    updateProcessButtonState();
}

async function generateNewSeed() {
    try {
        const targetCount = currentMode === 'auto' ? 12 : parseInt(currentMode);
        console.log(`Generating ${targetCount}-word seed phrase...`);

        const { invoke } = getTauriAPI();
        const generatedPhrase = await invoke('generate_seed_phrase', { wordCount: targetCount });
        currentWords = generatedPhrase.split(' ');
        editingIndex = -1;
        renderWords();
        updateValidationStatus();
        updateProcessButtonState();
        focusWordInput();

        console.log('✓ Seed phrase generated successfully');
    } catch (error) {
        console.error('❌ Failed to generate seed:', error);
        showError(`Failed to generate seed: ${error}`, 'Generation Error');
    }
}

function clearAllWords() {
    currentWords = [];
    editingIndex = -1;
    document.getElementById('wordInput').value = '';
    renderWords();
    hideAutocomplete();
    updateValidationStatus();
    updateProcessButtonState();
    focusWordInput();
}

function focusWordInput() {
    setTimeout(() => {
        document.getElementById('wordInput').focus();
    }, 0);
}

// File handling (placeholder for Phase 3)
function handleDragOver(e) {
    e.preventDefault();
    e.currentTarget.classList.add('drag-over');
}

function handleDragLeave(e) {
    e.currentTarget.classList.remove('drag-over');
}

async function handleDrop(e) {
    e.preventDefault();
    e.currentTarget.classList.remove('drag-over');

    const files = e.dataTransfer.files;
    if (files.length > 0) {
        try {
            console.log('File drop detected:', files[0].name);
            showError('File operations will be available in Phase 3', 'Feature Coming Soon');
        } catch (error) {
            console.error('File drop error:', error);
            showError('Failed to process dropped file', 'File Error');
        }
    }
}

async function handleFileSelect(e) {
    const file = e.target.files[0];
    if (file && file.type === 'text/plain') {
        try {
            console.log('File selected:', file.name);
            showError('File operations will be available in Phase 3', 'Feature Coming Soon');
        } catch (error) {
            console.error('File selection error:', error);
            showError('Failed to process selected file', 'File Error');
        }
    }
}

// Security functions
function togglePasswordVisibility(inputId, button) {
    const input = document.getElementById(inputId);
    if (input.type === 'password') {
        input.type = 'text';
        button.textContent = '🙈';
    } else {
        input.type = 'password';
        button.textContent = '👁️';
    }
}

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
    } else {
        button.textContent = '🛡️ Secure Screen: OFF';
        button.classList.remove('active');
        mainContent.classList.remove('secure-hidden');
        securityStatus.textContent = 'Secure mode ready';
    }
}

// Main processing function
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
        // Stage 1: Converting seed to binary
        statusText.textContent = 'Converting seed to binary representation...';
        progressFill.style.width = '20%';
        await new Promise(resolve => setTimeout(resolve, 300));

        // Stage 2: Key derivation
        statusText.textContent = 'Deriving keystream with Argon2id...';
        progressFill.style.width = '60%';

        // Call backend transformation
        console.log('Calling backend transform_seed_phrase...');
        const { invoke } = getTauriAPI();
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

        console.log('✓ Seed processing completed successfully');

    } catch (error) {
        console.error('❌ Processing error:', error);
        showError(`Processing failed: ${error}`, 'Processing Error');

        // Reset UI state
        progressBar.classList.remove('show');
        processButton.disabled = false;
        processButton.textContent = '🔄 Process Seed Phrase';
        statusText.textContent = 'Ready • Error occurred during processing';
    }
}

// Result handling functions
async function copyResultToClipboard() {
    try {
        const resultText = document.getElementById('resultText').textContent;
        await navigator.clipboard.writeText(resultText);

        const button = document.getElementById('copyResult');
        const originalText = button.textContent;

        button.textContent = '✅ Copied!';
        button.style.background = 'rgba(16, 185, 129, 0.2)';
        button.style.borderColor = '#10b981';
        button.style.color = '#6ee7b7';

        setTimeout(() => {
            button.textContent = originalText;
            button.style.background = '';
            button.style.borderColor = '';
            button.style.color = '';
        }, 2000);

        console.log('✓ Result copied to clipboard');
    } catch (error) {
        console.error('❌ Clipboard error:', error);
        showError('Failed to copy to clipboard', 'Clipboard Error');
    }
}

async function saveResultToFile() {
    try {
        showError('Save to file will be available in Phase 3', 'Feature Coming Soon');
    } catch (error) {
        console.error('❌ Save error:', error);
        showError('Failed to save file', 'Save Error');
    }
}

function processAnother() {
    // Clear current result and reset to input state
    const resultArea = document.getElementById('resultArea');
    resultArea.classList.remove('show');

    // Clear inputs
    clearAllWords();
    document.getElementById('passwordInput').value = '';

    // Reset button state
    updateProcessButtonState();

    // Focus input
    focusWordInput();

    console.log('✓ Ready for new processing');
}

// Utility functions
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
    console.log(`Error shown: ${title} - ${message}`);
}

// Address derivation functions (placeholder for Phase 4)
function togglePassphraseSection() {
    const content = document.getElementById('passphraseContent');
    const toggle = document.getElementById('passphraseToggle');
    const arrow = toggle.querySelector('.collapse-arrow');

    if (content.classList.contains('expanded')) {
        content.classList.remove('expanded');
        toggle.classList.remove('expanded');
        arrow.textContent = '▼';
    } else {
        content.classList.add('expanded');
        toggle.classList.add('expanded');
        arrow.textContent = '▲';
    }
}

function toggleNetworkSelection(button) {
    button.classList.toggle('active');
    // Network selection logic will be implemented in Phase 4
}

function deriveAddresses() {
    // Address derivation will be implemented in Phase 4
    showError('Address derivation will be available in Phase 4', 'Feature Coming Soon');
}

// Info modal for BIP39 passphrase explanation
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

// Memory cleanup on page unload
window.addEventListener('beforeunload', () => {
    // Clear sensitive variables
    currentWords = [];
    bip39WordList = [];

    // Clear input fields
    if (document.getElementById('wordInput')) {
        document.getElementById('wordInput').value = '';
    }
    if (document.getElementById('passwordInput')) {
        document.getElementById('passwordInput').value = '';
    }
    if (document.getElementById('bip39Passphrase')) {
        document.getElementById('bip39Passphrase').value = '';
    }

    console.log('✓ Memory cleanup completed');
});

// Console welcome message
console.log(`
╔══════════════════════════════════════╗
║          SCypher GUI v3.0            ║
║     XOR-based BIP39 Seed Cipher      ║
║                                      ║
║  Phase 2: Frontend Core Complete     ║
║  Backend Integration: Active         ║
╚══════════════════════════════════════╝
`);

// Export functions for debugging (development only)
if (window.location.hostname === 'localhost' || window.location.hostname === '127.0.0.1') {
    window.scypherDebug = {
        currentWords,
        validateSingleWord,
        validateCurrentPhrase,
        bip39WordList,
        getTauriAPI
    };
    console.log('🔧 Debug functions available in window.scypherDebug');
}
