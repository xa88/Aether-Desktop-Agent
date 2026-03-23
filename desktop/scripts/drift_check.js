const fs = require('fs');
const path = require('path');

const mainJsPath = path.join(__dirname, '../src/main.js');
const source = fs.readFileSync(mainJsPath, 'utf-8');

console.log('Scanning main.js for Security Baseline Configuration Drift...');

const REQUIRED_SETTINGS = {
    'nodeIntegration': 'false',
    'contextIsolation': 'true',
    'sandbox': 'true'
};

let driftDetected = false;

for (const [key, expectedValue] of Object.entries(REQUIRED_SETTINGS)) {
    // Basic regex to find e.g `nodeIntegration: false`
    const regex = new RegExp(`${key}\\s*:\\s*(${expectedValue}|true|false)`);
    const match = source.match(regex);
    
    if (!match) {
        console.error(`[FAIL] Mandatory security setting '${key}' is missing completely from main.js!`);
        driftDetected = true;
    } else if (match[1] !== expectedValue) {
        console.error(`[FAIL] Configuration Drift Detected: '${key}' was set to '${match[1]}', but MUST be '${expectedValue}'.`);
        driftDetected = true;
    } else {
        console.log(`[OK]   ${key} = ${expectedValue}`);
    }
}

if (driftDetected) {
    console.error('\nSECURITY BASELINE COMPROMISED. failing build.');
    process.exit(1);
} else {
    console.log('\nSecurity configuration is intact.');
    process.exit(0);
}
