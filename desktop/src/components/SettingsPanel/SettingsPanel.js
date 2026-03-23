export class SettingsPanel {
    constructor(containerId) {
        this.container = document.getElementById(containerId);
        this.render();
        this.bindEvents();
    }

    render() {
        this.container.innerHTML = `
            <div class="settings-overlay">
                <h2>Enterprise API Gateway Settings</h2>
                
                <div class="form-group">
                    <label>Task Routing Configuration</label>
                    <select id="task-route-type">
                        <option value="planning">Planning & Coordination</option>
                        <option value="execution">Execution & Fixing</option>
                        <option value="summarization">Context Summarization</option>
                    </select>
                </div>
                
                <div class="form-group">
                    <label>Base URL</label>
                    <input type="text" id="api-base-url" placeholder="https://llm.corp.local/v1">
                </div>
                
                <div class="form-group">
                    <label>Model ID</label>
                    <input type="text" id="api-model" placeholder="gpt-4 / claude-3 / minimax-xxx">
                </div>
                
                <div class="form-group">
                    <label>Bearer Token (Sends to OS Keychain)</label>
                    <input type="password" id="api-token" placeholder="••••••••••••••">
                </div>

                <div class="form-group" style="border-top: 1px solid #444; padding-top: 12px; margin-top: 12px;">
                    <label style="color: #999; font-size: 11px; text-transform: uppercase; letter-spacing: 1px;">Privacy & Telemetry</label>
                    <label style="display: flex; align-items: center; gap: 8px; cursor: pointer; margin-top: 8px;">
                        <input type="checkbox" id="telemetry-opt-in">
                        <span style="font-size: 12px; color: #aaa;">Allow anonymous crash reports (no personal data)</span>
                    </label>
                </div>

                <div class="form-actions">
                    <button id="btn-save-llm">Apply Route Policy</button>
                    <button id="btn-close-llm" class="secondary">Cancel</button>
                </div>
            </div>
        `;
    }

    bindEvents() {
        document.getElementById('btn-save-llm').addEventListener('click', async () => {
            const baseUrl = document.getElementById('api-base-url').value;
            const model = document.getElementById('api-model').value;
            const route = document.getElementById('task-route-type').value;

            if (!baseUrl || !model) {
                alert("Base URL and Model are administratively required");
                return;
            }
            
            try {
                const telemetryOptIn = document.getElementById('telemetry-opt-in').checked;
                // Mock storing token strictly using the OS KeyChain loop
                await window.ada.saveLlmConfig({ route, baseUrl, model, telemetryOptIn });
                alert("LLM Route bindings compiled to core!");
                this.container.style.display = 'none';
            } catch (e) {
                alert("Keychain commit blocked natively: " + e.message);
            }
        });
        
        document.getElementById('btn-close-llm').addEventListener('click', () => {
            this.container.style.display = 'none';
        });
    }
    
    show() {
        this.container.style.display = 'block';
    }
}
