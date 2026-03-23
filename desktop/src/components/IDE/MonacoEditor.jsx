import React, { useRef, useEffect } from 'react';
import * as monaco from 'monaco-editor';

const MonacoEditor = ({ content, language = 'rust', onChange, theme = 'vs-dark' }) => {
  const editorRef = useRef(null);
  const containerRef = useRef(null);

  useEffect(() => {
    if (containerRef.current) {
      editorRef.current = monaco.editor.create(containerRef.current, {
        value: content,
        language: language,
        theme: theme,
        automaticLayout: true,
        fontSize: 14,
        fontFamily: "'JetBrains Mono', 'Fira Code', monospace",
        minimap: { enabled: true },
        padding: { top: 16 },
        lineNumbersMinWidth: 40,
        renderLineHighlight: 'all',
        scrollbar: {
          vertical: 'visible',
          horizontal: 'visible',
          useShadows: false,
          verticalScrollbarSize: 8,
          horizontalScrollbarSize: 8,
        },
      });

      editorRef.current.onDidChangeModelContent(() => {
        if (onChange) {
          onChange(editorRef.current.getValue());
        }
      });
      
      // Listen for diagnostics if exposed globally or through a prop
      const handleDiagnostics = (_event, diagnostics) => {
        const model = editorRef.current.getModel();
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
      };
      
      if (window.ada && window.ada.onDiagnostics) {
        window.ada.onDiagnostics(handleDiagnostics);
      }

      return () => {
        editorRef.current.dispose();
      };
    }
  }, []);

  useEffect(() => {
    if (editorRef.current) {
      const model = editorRef.current.getModel();
      if (model && model.getValue() !== content) {
        editorRef.current.setValue(content);
        const newLanguage = language === 'rs' ? 'rust' : language;
        monaco.editor.setModelLanguage(model, newLanguage);
      }
    }
  }, [content, language]);

  return <div ref={containerRef} className="w-full h-full border-t border-white/5" />;
};

export default MonacoEditor;
