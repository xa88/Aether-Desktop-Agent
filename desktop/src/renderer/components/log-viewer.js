/**
 * LogViewer - Virtual scrolling component for displaying massive log files without freezing UI.
 */
class LogViewer {
    constructor(containerId, options = {}) {
        this.container = document.getElementById(containerId);
        if (!this.container) throw new Error(`LogViewer: Container #${containerId} not found`);

        this.itemHeight = options.itemHeight || 24;
        this.bufferLines = options.bufferLines || 20;

        this.logs = [];
        this.renderRoot = document.createElement('div');
        this.renderRoot.className = 'ada-log-viewer';
        this.renderRoot.style.position = 'relative';
        this.renderRoot.style.height = '100%';
        this.renderRoot.style.overflowY = 'auto';
        this.renderRoot.style.backgroundColor = 'var(--ada-bg-base)';
        this.renderRoot.style.border = '1px solid var(--ada-border-light)';
        this.renderRoot.style.borderRadius = 'var(--ada-radius-md)';
        this.renderRoot.style.fontFamily = 'var(--ada-font-family-mono)';
        this.renderRoot.style.fontSize = 'var(--ada-font-size-sm)';

        this.scrollSpace = document.createElement('div');
        this.scrollSpace.style.position = 'absolute';
        this.scrollSpace.style.top = '0';
        this.scrollSpace.style.left = '0';
        this.scrollSpace.style.width = '1px';

        this.viewport = document.createElement('div');
        this.viewport.style.position = 'sticky';
        this.viewport.style.top = '0';
        this.viewport.style.left = '0';
        this.viewport.style.width = '100%';
        this.viewport.style.overflow = 'hidden';

        this.renderRoot.appendChild(this.scrollSpace);
        this.renderRoot.appendChild(this.viewport);
        this.container.appendChild(this.renderRoot);

        this.renderRoot.addEventListener('scroll', () => this.render());
        
        // Setup ResizeObserver to rerender on resize
        this.resizeObserver = new ResizeObserver(() => this.render());
        this.resizeObserver.observe(this.renderRoot);
    }

    setLogs(logs) {
        // logs: Array of { level: 'INFO'|'ERROR'|'WARN', text: 'message' }
        this.logs = logs;
        this.scrollSpace.style.height = `${this.logs.length * this.itemHeight}px`;
        this.render();
    }

    addLog(log) {
        this.logs.push(log);
        this.scrollSpace.style.height = `${this.logs.length * this.itemHeight}px`;
        // Scroll to bottom if already at bottom
        const atBottom = this.renderRoot.scrollHeight - this.renderRoot.scrollTop <= this.renderRoot.clientHeight + this.itemHeight * 2;
        this.render();
        if (atBottom) {
            this.renderRoot.scrollTop = this.scrollSpace.clientHeight;
        }
    }

    render() {
        const scrollTop = this.renderRoot.scrollTop;
        const viewportHeight = this.renderRoot.clientHeight;
        
        const startIndex = Math.max(0, Math.floor(scrollTop / this.itemHeight) - this.bufferLines);
        const visibleCount = Math.ceil(viewportHeight / this.itemHeight) + (this.bufferLines * 2);
        const endIndex = Math.min(this.logs.length - 1, startIndex + visibleCount);

        const fragment = document.createDocumentFragment();

        for (let i = startIndex; i <= endIndex; i++) {
            const log = this.logs[i];
            const div = document.createElement('div');
            div.style.position = 'absolute';
            div.style.top = `${i * this.itemHeight}px`;
            div.style.left = '0';
            div.style.width = '100%';
            div.style.height = `${this.itemHeight}px`;
            div.style.lineHeight = `${this.itemHeight}px`;
            div.style.padding = '0 var(--ada-spacing-2)';
            div.style.whiteSpace = 'pre';
            div.style.boxSizing = 'border-box';
            div.style.borderBottom = '1px solid var(--ada-bg-surface)';

            // Set colors based on log level
            if (log.level === 'ERROR') {
                div.style.color = 'var(--ada-color-error)';
                div.style.backgroundColor = 'var(--ada-color-error-bg)';
            } else if (log.level === 'WARN') {
                div.style.color = 'var(--ada-color-warning)';
                div.style.backgroundColor = 'var(--ada-color-warning-bg)';
            } else if (log.level === 'SUCCESS') {
                div.style.color = 'var(--ada-color-success)';
            } else {
                div.style.color = 'var(--ada-text-secondary)';
            }

            const levelSpan = document.createElement('span');
            levelSpan.style.display = 'inline-block';
            levelSpan.style.width = '70px';
            levelSpan.style.fontWeight = '600';
            levelSpan.textContent = `[${log.level || 'INFO'}]`;

            const textSpan = document.createElement('span');
            textSpan.textContent = log.text;

            div.appendChild(levelSpan);
            div.appendChild(textSpan);
            fragment.appendChild(div);
        }

        this.viewport.innerHTML = '';
        this.viewport.appendChild(fragment);
    }
}

// Export if in module environment, else available globally
if (typeof module !== 'undefined' && module.exports) {
    module.exports = LogViewer;
}
