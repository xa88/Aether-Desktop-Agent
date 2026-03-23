/**
 * DiffViewer - Component to display text differences using the ADA design tokens.
 */
class DiffViewer {
    constructor(containerId) {
        this.container = document.getElementById(containerId);
        if (!this.container) throw new Error(`DiffViewer: Container #${containerId} not found`);

        this.root = document.createElement('div');
        this.root.className = 'ada-diff-viewer ada-card';
        this.root.style.fontFamily = 'var(--ada-font-family-mono)';
        this.root.style.fontSize = 'var(--ada-font-size-sm)';
        this.root.style.overflowX = 'auto';
        this.root.style.whiteSpace = 'pre';

        this.container.appendChild(this.root);
    }

    /**
     * @param {Array} diffLines - Array of { type: 'add'|'remove'|'unchanged', text: 'line content' }
     */
    setDiff(diffLines) {
        this.root.innerHTML = '';
        
        diffLines.forEach(line => {
            const row = document.createElement('div');
            row.style.padding = '2px var(--ada-spacing-3)';
            
            if (line.type === 'add') {
                row.style.backgroundColor = 'var(--ada-color-success-bg)';
                row.style.color = 'var(--ada-color-success)';
                row.textContent = `+ ${line.text}`;
            } else if (line.type === 'remove') {
                row.style.backgroundColor = 'var(--ada-color-error-bg)';
                row.style.color = 'var(--ada-color-error)';
                row.textContent = `- ${line.text}`;
            } else {
                row.style.color = 'var(--ada-text-secondary)';
                row.textContent = `  ${line.text}`;
            }
            
            this.root.appendChild(row);
        });
    }
}

if (typeof module !== 'undefined' && module.exports) {
    module.exports = DiffViewer;
}
