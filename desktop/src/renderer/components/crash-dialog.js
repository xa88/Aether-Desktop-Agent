/**
 * CrashDialog - Global overlay component to show critical errors using ADA guidelines.
 */
class CrashDialog {
    constructor() {
        this.overlay = document.createElement('div');
        this.overlay.className = 'ada-dialog-overlay';
        this.overlay.style.display = 'none';

        this.dialog = document.createElement('div');
        this.dialog.className = 'ada-dialog';
        this.dialog.style.border = '1px solid var(--ada-color-error)';

        this.header = document.createElement('div');
        this.header.className = 'ada-dialog-header';
        this.header.style.backgroundColor = 'var(--ada-color-error-bg)';
        this.header.style.color = 'var(--ada-color-error)';
        this.header.innerHTML = `<h3><span class="ada-badge error" style="margin-right: 8px;">CRITICAL ERROR</span> Agent Crash Detected</h3>`;

        this.content = document.createElement('div');
        this.content.className = 'ada-dialog-content';

        this.messageEl = document.createElement('p');
        this.messageEl.style.fontWeight = '500';
        this.messageEl.style.marginBottom = 'var(--ada-spacing-3)';

        this.stackEl = document.createElement('pre');
        this.stackEl.style.backgroundColor = 'var(--ada-bg-base)';
        this.stackEl.style.padding = 'var(--ada-spacing-3)';
        this.stackEl.style.borderRadius = 'var(--ada-radius-sm)';
        this.stackEl.style.overflowX = 'auto';
        this.stackEl.style.fontSize = 'var(--ada-font-size-xs)';
        this.stackEl.style.color = 'var(--ada-text-secondary)';

        this.content.appendChild(this.messageEl);
        this.content.appendChild(this.stackEl);

        this.footer = document.createElement('div');
        this.footer.className = 'ada-dialog-footer';

        this.copyBtn = document.createElement('button');
        this.copyBtn.className = 'ada-btn';
        this.copyBtn.textContent = 'Copy to Clipboard';
        this.copyBtn.onclick = () => {
            navigator.clipboard.writeText(`${this.messageEl.textContent}\n\n${this.stackEl.textContent}`);
            this.copyBtn.textContent = 'Copied!';
            setTimeout(() => this.copyBtn.textContent = 'Copy to Clipboard', 2000);
        };

        this.closeBtn = document.createElement('button');
        this.closeBtn.className = 'ada-btn primary danger';
        this.closeBtn.textContent = 'Acknowledge & Restart';
        this.closeBtn.onclick = () => this.hide();

        this.footer.appendChild(this.copyBtn);
        this.footer.appendChild(this.closeBtn);

        this.dialog.appendChild(this.header);
        this.dialog.appendChild(this.content);
        this.dialog.appendChild(this.footer);
        this.overlay.appendChild(this.dialog);

        document.body.appendChild(this.overlay);
    }

    show(message, stackTrace) {
        this.messageEl.textContent = message;
        this.stackEl.textContent = stackTrace;
        this.overlay.style.display = 'flex';
    }

    hide() {
        this.overlay.style.display = 'none';
        // Typically trigger an IPC restart message here
    }
}

if (typeof module !== 'undefined' && module.exports) {
    module.exports = CrashDialog;
}
