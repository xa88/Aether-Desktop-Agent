const { contextBridge, ipcRenderer } = require('electron');

contextBridge.exposeInMainWorld('ada', {
    runPlan: (path) => ipcRenderer.invoke('run-plan', path),
    readFile: (path) => ipcRenderer.invoke('read-file', path),
    writeFile: (path, content) => ipcRenderer.invoke('write-file', path, content),
    onDiagnostics: (callback) => ipcRenderer.on('ide:diagnostics', callback),
    onPlanStdout: (callback) => ipcRenderer.on('plan-stdout', callback),
    saveLlmConfig: (config) => ipcRenderer.invoke('save-llm-config', config),
    listFiles: (path) => ipcRenderer.invoke('list-files', path),
    captureVoice: () => ipcRenderer.invoke('capture-voice'),
    readAuditLog: () => ipcRenderer.invoke('read-audit-log'),
    browserNavigate: (url) => ipcRenderer.invoke('browser-navigate', url),
    runInSandbox: (code, language) => ipcRenderer.invoke('run-in-sandbox', { code, language }),
    onTerminalData: (callback) => ipcRenderer.on('terminal-data', (event, data) => callback(data)),
    
    // Phase 6: Backend Bridging
    speakText: (text) => ipcRenderer.invoke('speak-text', text),
    vaultSetSecret: (key, value) => ipcRenderer.invoke('vault-set-secret', { key, value }),
    vaultGetSecret: (key) => ipcRenderer.invoke('vault-get-secret', key),
    onAuditEvent: (callback) => ipcRenderer.on('audit-event', (event, data) => callback(data)),
    getHealthMetrics: () => ipcRenderer.invoke('get-health-metrics'),
    getSandboxStatus: () => ipcRenderer.invoke('get-sandbox-status'),
    getContextStats: () => ipcRenderer.invoke('get-context-stats'),
    searchPlugins: (query) => ipcRenderer.invoke('search-plugins', query),
    installPlugin: (id) => ipcRenderer.invoke('install-plugin', id),
    getSwarmStatus: () => ipcRenderer.invoke('get-swarm-status'),
    restartAgent: (id) => ipcRenderer.invoke('restart-agent', id),
    testLlmConnection: (config) => ipcRenderer.invoke('test-llm-connection', config),
  getClusterNodes: () => ipcRenderer.invoke('get-cluster-nodes'),
  getStoredExperiences: () => ipcRenderer.invoke('get-stored-experiences'),
});

window.onerror = function (message, source, lineno, colno, error) {
    const stack = error ? error.stack : message;
    ipcRenderer.send('renderer-crash', `[${source}:${lineno}:${colno}] ${stack}`);
    return true; // Prevents default browser logging
};
