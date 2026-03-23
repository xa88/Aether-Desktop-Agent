// LogViewer Web Component
export class LogViewer {
    constructor(containerId) {
        this.container = document.getElementById(containerId);
        this.container.innerHTML = `
            <div class="log-viewer-ui">
                <div class="log-header">
                    <span>Audit Pipeline Logs</span>
                    <div>
                        <button id="lv-copy">Copy Trace</button>
                        <button id="lv-filter-err">Filter Errors</button>
                    </div>
                </div>
                <div class="log-stream" id="lv-stream"></div>
            </div>
        `;
        this.stream = document.getElementById('lv-stream');
        this.filterErrors = false;
        this.logs = [];
        
        document.getElementById('lv-copy').addEventListener('click', () => {
             navigator.clipboard.writeText(this.logs.map(l => l.text).join('\n'));
             alert("Copied logs to clipboard");
        });
        
        document.getElementById('lv-filter-err').addEventListener('click', (e) => {
             this.filterErrors = !this.filterErrors;
             e.target.style.background = this.filterErrors ? '#d32f2f' : '#007acc';
             this.render();
        });
    }

    appendLog(jsonlString) {
        try {
            const event = typeof jsonlString === 'string' ? JSON.parse(jsonlString) : jsonlString;
            let cssClass = 'log-line';
            let blockText = `[${event.tool}:${event.action}] `;
            
            if (event.result && event.result.Failure) {
                cssClass += ' log-error';
                blockText += `FAIL: ${event.result.Failure.reason}`;
            } else if (event.result && event.result.Success) {
                cssClass += ' log-success';
                blockText += `OK`;
            } else {
                blockText += JSON.stringify(event.result || {});
            }
            
            this.logs.push({ raw: event, text: blockText, isErr: cssClass.includes('log-error') });
            
            // Auto virtual truncation array retention (keep tail 500 lines locally inside DOM rendering limits)
            if (this.logs.length > 500) this.logs.shift();
            
            this.render();
        } catch(e) {
            console.error("LogViewer parse failure:", e);
        }
    }
    
    render() {
        this.stream.innerHTML = '';
        for (const log of this.logs) {
            if (this.filterErrors && !log.isErr) continue;
            
            const div = document.createElement('div');
            div.className = log.isErr ? 'log-line log-error' : 'log-line log-success';
            div.innerText = log.text;
            this.stream.appendChild(div);
        }
        this.stream.scrollTop = this.stream.scrollHeight;
    }
}
