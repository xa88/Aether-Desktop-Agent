const { app, BrowserWindow, ipcMain, dialog } = require('electron');
const path = require('path');
const fs = require('fs');
const https = require('https');
const { spawn, execSync } = require('child_process');
const LSPClient = require('./lsp/client');

const PLUGIN_DIR = path.join(app.getPath('userData'), 'plugins');
if (!fs.existsSync(PLUGIN_DIR)) fs.mkdirSync(PLUGIN_DIR, { recursive: true });

ipcMain.handle('search-plugins', async (event, query) => {
  return new Promise((resolve, reject) => {
    const url = `https://open-vsx.org/api/-/search?q=${encodeURIComponent(query)}`;
    https.get(url, (res) => {
      let data = '';
      res.on('data', (chunk) => data += chunk);
      res.on('end', () => {
        try {
          const json = JSON.parse(data);
          const results = (json.results || []).map(r => ({
            id: r.namespace + '.' + r.name,
            name: r.displayName || r.name,
            description: r.description,
            version: r.version,
            publisher: r.namespace,
            icon: r.iconUrl || 'https://open-vsx.org/assets/icons/extension-icon.png',
            downloadUrl: r.files?.download
          }));
          resolve(results);
        } catch (e) { resolve([]); }
      });
    }).on('error', (err) => resolve([]));
  });
});

ipcMain.handle('install-plugin', async (event, plugin) => {
  const targetDir = path.join(PLUGIN_DIR, plugin.id);
  if (fs.existsSync(targetDir)) return { success: true, message: 'Already installed' };

  try {
    const vsixPath = path.join(app.getPath('temp'), `${plugin.id}.vsix`);
    
    // Use PowerShell to download and extract
    const psCommand = `
      $ProgressPreference = 'SilentlyContinue'
      Invoke-WebRequest -Uri "${plugin.downloadUrl}" -OutFile "${vsixPath}"
      Expand-Archive -Path "${vsixPath}" -DestinationPath "${targetDir}" -Force
      Remove-Item "${vsixPath}"
    `;
    
    execSync(`powershell -Command "${psCommand}"`);
    return { success: true };
  } catch (err) {
    console.error('Plugin Install Failed:', err);
    return { success: false, error: err.message };
  }
});

// -----------------------------------------------------------------------------
// P23.T02: Avoid Silent Exits (Boot Fallback + Global Crash Dialog)
// -----------------------------------------------------------------------------
function dumpCrash(errorStr) {
  const logDir = path.join(__dirname, '../../logs');
  if (!fs.existsSync(logDir)) fs.mkdirSync(logDir, { recursive: true });
  const dumpFile = path.join(logDir, `crash_${Date.now()}.txt`);
  fs.writeFileSync(dumpFile, errorStr, 'utf8');
  return dumpFile;
}

process.on('uncaughtException', (err) => {
  const file = dumpCrash(err.stack || err.toString());
  dialog.showErrorBox('ADA Fatal Error', `The Desktop Agent encountered an unrecoverable node.js trace.\n\nCrash dumped to: ${file}\n\n${err.message}`);
  process.exit(1);
});

process.on('unhandledRejection', (reason, promise) => {
  const file = dumpCrash(String(reason));
  dialog.showErrorBox('ADA Unhandled Promise', `Asynchronous promise rejected globally.\n\nCrash dumped to: ${file}\n\n${reason}`);
  process.exit(1);
});

let mainWindow;
let lspClient;

function createWindow() {
  mainWindow = new BrowserWindow({
    width: 1400,
    height: 900,
    webPreferences: {
      preload: path.join(__dirname, 'preload.js'),
      contextIsolation: true,
      nodeIntegration: false,
      sandbox: true,
    },
  });

  const { session } = require('electron');
  session.defaultSession.webRequest.onHeadersReceived((details, callback) => {
    callback({
      responseHeaders: {
        ...details.responseHeaders,
        'Content-Security-Policy': ["default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; object-src 'none'; img-src 'self' data:;"]
      }
    })
  });

  // In development, you might want to load from the Vite dev server:
  // mainWindow.loadURL('http://localhost:5173');
  
  // For now, we will point to the built React index or the root index if we use Vite's build
  mainWindow.loadFile(path.join(__dirname, '../dist-renderer/index.html')); 

  // Initialize LSP
  const workspaceRoot = path.join(__dirname, '../../');
  lspClient = new LSPClient('rust-analyzer', workspaceRoot);

  lspClient.onDiagnostics = (params) => {
    mainWindow.webContents.send('ide:diagnostics', params.diagnostics);
  };

  lspClient.start();
}

app.whenReady().then(() => {
  createWindow();

  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) {
      createWindow();
    }
  });
});

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

const { spawn } = require('child_process');

// IPC handlers for Core Runtime communication
ipcMain.handle('run-plan', async (event, planPath) => {
  console.log(`Running plan: ${planPath}`);

  return new Promise((resolve, reject) => {
    // Spawn the Rust CLI
    // In production, we would use the compiled binary path depending on the platform.
    const adaProcess = spawn('cargo', ['run', '-p', 'ada-cli', '--', planPath], {
      cwd: path.join(__dirname, '../../') // ADA workspace root
    });

    let stdoutData = '';
    let stderrData = '';

    adaProcess.stdout.on('data', (data) => {
      stdoutData += data.toString();
      event.sender.send('plan-stdout', data.toString()); 
    });

    adaProcess.stderr.on('data', (data) => {
      stderrData += data.toString();
    });

    adaProcess.on('close', (code) => {
      console.log(`ada-cli exited with code ${code}`);

      resolve({
        success: code === 0,
        exitCode: code,
        stdout: stdoutData,
        stderr: stderrData
      });
    });

    adaProcess.on('error', (err) => {
      console.error('Failed to start ada-cli', err);
      reject(err);
    });
  });
});
// --- IDE IPC Handlers ---

ipcMain.handle('save-llm-config', async (event, config) => {
  console.log(`Saving Dual-Model Config: Director=${config.directorModel}, Worker=${config.workerModel}`);
  const profilePath = path.join(__dirname, '..', '..', 'core', 'cli', 'profile.yaml');
  const profile = {
    llm: {
      base_url: config.baseUrl,
      api_key: config.apiKey,
      director_model: config.directorModel,
      worker_model: config.workerModel
    }
  };
  fs.writeFileSync(profilePath, yaml.dump(profile));
  return { success: true };
});

ipcMain.handle('test-llm-connection', async (event, { baseUrl, apiKey, model }) => {
  console.log(`Testing connection for ${model} at ${baseUrl}`);
  try {
    // Phase 17: Dummy validation call to OpenAI-compatible /v1/models or similar
    // For now, simulator a request
    await new Promise(resolve => setTimeout(resolve, 1500));
    
    // Simple logic: if apiKey starts with "sk-", it's "valid" for the mock
    if (apiKey.length < 5) throw new Error("Invalid API Key format");
    
    return { success: true };
  } catch (err) {
    return { success: false, error: err.message };
  }
});

ipcMain.handle('read-file', async (event, relativePath) => {
  const fullPath = path.join(__dirname, '../../', relativePath);
  const content = fs.readFileSync(fullPath, 'utf8');

  // Notify LSP that file is opened
  lspClient.didOpen(fullPath, content);

  return content;
});

ipcMain.handle('write-file', async (event, relativePath, content) => {
  const fullPath = path.join(__dirname, '../../', relativePath);
  fs.writeFileSync(fullPath, content, 'utf8');

  // Notify LSP of changes
  lspClient.didChange(fullPath, content);

  return true;
});

ipcMain.handle('list-files', async (event, relativePath = '') => {
  const rootPath = path.join(__dirname, '../../', relativePath);
  
  async function walk(dir) {
    const files = fs.readdirSync(dir, { withFileTypes: true });
    const result = [];
    
    for (const file of files) {
      if (file.name === 'node_modules' || file.name === '.git' || file.name === 'target') continue;
      
      const res = path.resolve(dir, file.name);
      const rel = path.relative(path.join(__dirname, '../../'), res);
      
      if (file.isDirectory()) {
        result.push({
          name: file.name,
          path: rel,
          type: 'directory',
          children: await walk(res)
        });
      } else {
        result.push({
          name: file.name,
          path: rel,
          type: 'file'
        });
      }
    }
    return result;
  }

  return await walk(rootPath);
});

ipcMain.handle('capture-voice', async (event) => {
  return new Promise((resolve, reject) => {
    const adaProcess = spawn('cargo', ['run', '-p', 'ada-cli', '--', 'listen'], {
      cwd: path.join(__dirname, '../../')
    });

    let transcript = '';
    adaProcess.stdout.on('data', (data) => {
      const text = data.toString();
      const match = text.match(/TRANSCRIPT: (.*)/);
      if (match) {
        transcript = match[1].trim();
      }
    });

    adaProcess.on('close', (code) => {
      if (code === 0) resolve(transcript);
      else reject(new Error(`Voice capture exited with code ${code}`));
    });

    adaProcess.on('error', reject);
  });
});

ipcMain.handle('read-audit-log', async (event) => {
  const auditPath = path.join(__dirname, '../../runs/audit.jsonl');
  if (!fs.existsSync(auditPath)) return [];
  
  const content = fs.readFileSync(auditPath, 'utf8');
  return content.split('\n')
    .filter(line => line.trim())
    .map(line => {
      try { return JSON.parse(line); }
      catch (e) { return null; }
    })
    .filter(Boolean);
});

ipcMain.handle('browser-navigate', async (event, url) => {
  console.log(`Browsing to: ${url}`);
  // Simulated Playwright result
  return {
    success: true,
    title: 'ADA Project - GitHub',
    screenshot: null, // Would be a base64 string
  };
});

ipcMain.handle('manual-plan-import', async () => {
  return { success: true, message: 'Plan imported successfully' };
});
// Phase 6: Backend Bridging Handlers
ipcMain.handle('speak-text', async (event, text) => {
  console.log(`TTS Request: ${text}`);
  // Spawn the Rust CLI to handle TTS
  return new Promise((resolve, reject) => {
    const adaProcess = spawn('cargo', ['run', '-p', 'ada-cli', '--', 'tts', text], {
      cwd: path.join(__dirname, '../../')
    });
    let output = '';
    adaProcess.stdout.on('data', (data) => output += data.toString());
    adaProcess.stderr.on('data', (data) => console.error(data.toString()));
    adaProcess.on('close', (code) => {
      if (code === 0) resolve(output);
      else reject(new Error(`TTS failed with code ${code}`));
    });
  });
});

ipcMain.handle('vault-set-secret', async (event, { key, value }) => {
  console.log(`Vault Set: ${key}`);
  return new Promise((resolve, reject) => {
    const adaProcess = spawn('cargo', ['run', '-p', 'ada-cli', '--', 'credentials', 'set', key, value], {
      cwd: path.join(__dirname, '../../')
    });
    adaProcess.on('close', (code) => code === 0 ? resolve() : reject(new Error('Vault Set Failed')));
  });
});

ipcMain.handle('get-health-metrics', async () => {
  const metricsPath = path.join(__dirname, '../../runs/metrics.json');
  if (!fs.existsSync(metricsPath)) {
    return { cache_hits: 0, cache_misses: 0, time_saved_ms: 0, cpu_usage: 12, memory_usage: 45 };
  }
  try {
    const data = JSON.parse(fs.readFileSync(metricsPath, 'utf8'));
    return {
      ...data,
      cpu_usage: Math.floor(Math.random() * 15 + 5), // Mocked dynamic system metrics
      memory_usage: Math.floor(Math.random() * 10 + 40),
    };
  } catch (e) {
    return { cache_hits: 0, cache_misses: 0, time_saved_ms: 0, cpu_usage: 0, memory_usage: 0 };
  }
});

ipcMain.handle('get-sandbox-status', async () => {
  // In a real scenario, this would check Docker status or current core config
  return { 
    provider: 'Docker (Standard)', 
    status: 'Ready', 
    container_id: 'ada-sandbox-v1',
    is_isolated: true 
  };
});
ipcMain.handle('vault-get-secret', async (event, key) => {
  console.log(`Vault Get: ${key}`);
  return new Promise((resolve, reject) => {
    const adaProcess = spawn('cargo', ['run', '-p', 'ada-cli', '--', 'credentials', 'get', key], {
      cwd: path.join(__dirname, '../../')
    });
    let output = '';
    adaProcess.stdout.on('data', d => output += d.toString());
    adaProcess.on('close', (code) => code === 0 ? resolve(output.trim()) : reject(new Error('Vault Get Failed')));
  });
});

ipcMain.handle('search-plugins', async (event, query) => {
  // Mocked Open VSX / Marketplace results
  const allPlugins = [
    { id: 'antigravity', name: 'Antigravity AI', version: '2.1.0', source: 'Internal', description: 'Advanced AI Chat Panel for IDEs.', permissions: ['ide_access', 'chat_intercept'] },
    { id: 'vscode-python', name: 'Python', version: '2024.2.0', source: 'Open VSX', description: 'IntelliSense, linting, and debugging for Python.', permissions: ['fs_read', 'shell_exec'] },
    { id: 'git-lens', name: 'GitLens', version: '14.0.0', source: 'Open VSX', description: 'Supercharge Git in ADA.', permissions: ['git_read', 'git_write'] },
    { id: 'rust-analyzer', name: 'Rust Analyzer', version: '0.3.1850', source: 'Open VSX', description: 'Rust language support for ADA IDE.', permissions: ['fs_read', 'fs_write', 'shell_exec'] }
  ];
  if (!query) return allPlugins;
  return allPlugins.filter(p => p.name.toLowerCase().includes(query.toLowerCase()));
});

ipcMain.handle('install-plugin', async (event, pluginId) => {
  console.log(`Installing plugin: ${pluginId}`);
  // Simulated delay
  await new Promise(resolve => setTimeout(resolve, 1500));
  return { success: true, message: `${pluginId} installed successfully in sandbox.` };
});

ipcMain.handle('get-swarm-status', async () => {
  // Phase 10-15: Mocked Swarm Status for "Build Web App" task
  return {
    active_agents: 4,
    total_tasks: 6,
    completed_tasks: 2,
    workers: [
      { id: 'worker-db', name: 'DatabaseArchitect', status: 'Success', current_task: 'Initialize SQLite schema', progress: 1.0 },
      { id: 'worker-be', name: 'BackendDev', status: 'Executing', current_task: 'Implement REST endpoints in Rust', progress: 0.42 },
      { id: 'worker-fe', name: 'FrontendDev', status: 'Executing', current_task: 'Build React dashboard components', progress: 0.28 },
      { id: 'worker-qa', name: 'QA-Automator', status: 'Planning', current_task: 'Write Playwright test scripts', progress: 0.10 },
      { id: 'worker-devops', name: 'DevOps-Engineer', status: 'Idle', current_task: 'Configure Docker deployment', progress: 0.0 },
      { id: 'director', name: 'Director', status: 'Executing', current_task: 'Coordinating Frontend/Backend sync', progress: 0.45 }
    ]
  };
});

ipcMain.handle('get-stored-experiences', async () => {
  // Phase 18: Mocked Perpetual Memory Discovery
  return [
    { signature: 'Build Web App with SQLite', timestamp: Date.now() - 86400000, model: 'GPT-4o', success: true },
    { signature: 'Refactor Core Orchestrator', timestamp: Date.now() - 172800000, model: 'Phi-3', success: true },
    { signature: 'Deploy Cluster Node', timestamp: Date.now() - 3600000, model: 'GPT-4o', success: true }
  ];
});

ipcMain.handle('run-in-sandbox', async (event, content, language) => {
  console.log(`Running ${language} code in sandbox...`);
  // Simulated Sandbox Execution
  event.sender.send('terminal-data', `\x1b[1;32mStarting execution of ${language} script...\x1b[0m\r\n`);
  
  return new Promise((resolve) => {
    setTimeout(() => {
      event.sender.send('terminal-data', `Processing 16,384 tokens...\r\n`);
      setTimeout(() => {
        event.sender.send('terminal-data', `\x1b[1;32mExecution successful.\x1b[0m\r\n\x1b[1;35mroot@ada-sandbox:~\x1b[0m# `);
        resolve({ success: true, output: 'Execution successful.' });
      }, 1000);
    }, 500);
  });
});

ipcMain.handle('spawn-ide-sandbox', async (event) => {
  console.log("Spawning Isolated Dev Sandbox mapping port 8080...");
  
  // ADA CLI/Rust invokes Docker sandbox mapping port 8080 internally.
  const ideWindow = new BrowserWindow({
    width: 1200,
    height: 800,
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true,
      sandbox: true,
    }
  });

  // Safe isolated UI constrained explicitly to localhost bypassing the Host's firewall exposure
  ideWindow.loadURL('http://127.0.0.1:8080');
  
  return true;
});

// IPC handler to capture Frontend Render Crashes natively onto the OS
ipcMain.on('renderer-crash', (event, errorStack) => {
  const file = dumpCrash(errorStack);
  dialog.showErrorBox('Frontend Render Crash', `A bug occurred structurally in the IDE WebView natively.\n\nDumped to: ${file}\n\n${errorStack}`);
});

// -----------------------------------------------------------------------------
// P24.T02: Enterprise Config Keychain Mock bindings
// -----------------------------------------------------------------------------
ipcMain.handle('save-llm-config', async (event, config) => {
  console.log("Saving new Enterprise Model routing table securely...");
  // Simulated native Keytar/Keychain hook storing base_url, model_id, etc.
  const routeFile = path.join(__dirname, '../../profile.yaml');
  fs.appendFileSync(routeFile, `\n# LLM Overrides\nllm_override: ${JSON.stringify(config)}\n`);
  return true;
});
