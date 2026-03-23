let editor;

// Initialize Monaco Editor
require(['vs/editor/editor.main'], function () {
    editor = monaco.editor.create(document.getElementById('editor-container'), {
        value: '// Select a file from the sidebar to start editing',
        language: 'rust',
        theme: 'vs-dark',
        automaticLayout: true,
        fontSize: 14,
        minimap: { enabled: true }
    });

    console.log("Monaco Editor initialized");
});

// File Sidebar logic
document.querySelectorAll('.file-item').forEach(item => {
    item.addEventListener('click', async () => {
        const filePath = item.getAttribute('data-path');
        document.querySelectorAll('.file-item').forEach(i => i.classList.remove('active'));
        item.classList.add('active');

        try {
            const content = await window.ada.readFile(filePath);
            const model = monaco.editor.createModel(content, 'rust');
            editor.setModel(model);
        } catch (err) {
            console.error("Failed to read file:", err);
            alert("Error reading file: " + err.message);
        }
    });
});

// Save logic
async function saveCurrentFile() {
    const model = editor.getModel();
    if (!model) return;

    const activeItem = document.querySelector('.file-item.active');
    if (!activeItem) return;

    const filePath = activeItem.getAttribute('data-path');
    const content = model.getValue();

    try {
        await window.ada.writeFile(filePath, content);
        console.log("File saved successfully:", filePath);
    } catch (err) {
        console.error("Failed to save file:", err);
        alert("Error saving file: " + err.message);
    }
}

document.getElementById('save-btn').addEventListener('click', saveCurrentFile);

// Keyboard shortcuts
window.addEventListener('keydown', (e) => {
    if (e.ctrlKey && e.key === 's') {
        e.preventDefault();
        saveCurrentFile();
    }
});

// Listen for LSP diagnostics
window.ada.onDiagnostics((event, diagnostics) => {
    console.log("Received diagnostics:", diagnostics);
    const model = editor.getModel();
    if (!model) return;

    const markers = diagnostics.map(d => ({
        severity: d.severity === 1 ? monaco.MarkerSeverity.Error : monaco.MarkerSeverity.Warning,
        message: d.message,
        startLineNumber: d.range.start.line + 1,
        startColumn: d.range.start.character + 1,
        endLineNumber: d.range.end.line + 1,
        endColumn: d.range.end.character + 1,
    }));

    monaco.editor.setModelMarkers(model, "lsp", markers);
});
