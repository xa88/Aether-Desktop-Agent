const { spawn } = require('child_process');
const path = require('path');

class LSPClient {
    constructor(serverPath, workspaceRoot) {
        this.serverPath = serverPath;
        this.workspaceRoot = workspaceRoot;
        this.process = null;
        this.idCounter = 0;
        this.callbacks = new Map();
        this.buffer = '';
        this.onDiagnostics = null;
    }

    start() {
        console.log(`Starting LSP Server: ${this.serverPath}`);
        this.process = spawn(this.serverPath, [], {
            cwd: this.workspaceRoot,
            env: { ...process.env, RUST_BACKTRACE: '1' }
        });

        this.process.stdout.on('data', (data) => this.handleData(data));
        this.process.stderr.on('data', (data) => {
            // console.error(`LSP STDERR: ${data}`);
        });

        this.process.on('close', (code) => {
            console.log(`LSP Server exited with code ${code}`);
        });

        // Send Initialize
        this.sendRequest('initialize', {
            processId: process.pid,
            rootUri: `file://${this.workspaceRoot.replace(/\\/g, '/')}`,
            capabilities: {
                textDocument: {
                    publishDiagnostics: { relatedInformation: true }
                }
            }
        });
    }

    handleData(data) {
        this.buffer += data.toString();
        while (true) {
            const contentLengthMatch = this.buffer.match(/Content-Length: (\d+)\r\n\r\n/);
            if (!contentLengthMatch) break;

            const contentLength = parseInt(contentLengthMatch[1]);
            const headerLength = contentLengthMatch[0].length;

            if (this.buffer.length < headerLength + contentLength) break;

            const messageJson = this.buffer.slice(headerLength, headerLength + contentLength);
            this.buffer = this.buffer.slice(headerLength + contentLength);

            try {
                const message = JSON.parse(messageJson);
                this.dispatchMessage(message);
            } catch (e) {
                console.error("LSP Parse Error:", e);
            }
        }
    }

    dispatchMessage(msg) {
        if (msg.id !== undefined) {
            const cb = this.callbacks.get(msg.id);
            if (cb) {
                cb(msg.result || msg.error);
                this.callbacks.delete(msg.id);
            }
        } else if (msg.method === 'textDocument/publishDiagnostics') {
            if (this.onDiagnostics) {
                this.onDiagnostics(msg.params);
            }
        } else if (msg.method === 'window/logMessage') {
            // console.log("LSP Log:", msg.params.message);
        }
    }

    sendRequest(method, params) {
        const id = this.idCounter++;
        const msg = JSON.stringify({ jsonrpc: '2.0', id, method, params });
        this.write(msg);
        return new Promise(resolve => this.callbacks.set(id, resolve));
    }

    sendNotification(method, params) {
        const msg = JSON.stringify({ jsonrpc: '2.0', method, params });
        this.write(msg);
    }

    write(msg) {
        const header = `Content-Length: ${Buffer.byteLength(msg, 'utf8')}\r\n\r\n`;
        this.process.stdin.write(header + msg);
    }

    didOpen(filePath, text) {
        this.sendNotification('textDocument/didOpen', {
            textDocument: {
                uri: `file://${path.resolve(filePath).replace(/\\/g, '/')}`,
                languageId: 'rust',
                version: 1,
                text: text
            }
        });
    }

    didChange(filePath, text) {
        this.sendNotification('textDocument/didChange', {
            textDocument: {
                uri: `file://${path.resolve(filePath).replace(/\\/g, '/')}`,
                version: 2
            },
            contentChanges: [{ text }]
        });
    }
}

module.exports = LSPClient;
