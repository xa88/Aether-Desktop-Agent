import React, { useEffect, useRef } from 'react';
import { Terminal as XTerm } from 'xterm';
import { FitAddon } from 'xterm-addon-fit';
import 'xterm/css/xterm.css';

const TerminalView = ({ onData }) => {
  const terminalRef = useRef(null);
  const xtermRef = useRef(null);

  useEffect(() => {
    if (terminalRef.current && !xtermRef.current) {
      const term = new XTerm({
        cursorBlink: true,
        fontSize: 13,
        fontFamily: "'JetBrains Mono', 'Fira Code', monospace",
        theme: {
          background: 'rgba(0, 0, 0, 0)',
          foreground: '#aether-ghost',
          cursor: '#6366f1',
        },
        allowTransparency: true,
      });

      const fitAddon = new FitAddon();
      term.loadAddon(fitAddon);
      term.open(terminalRef.current);
      fitAddon.fit();

      if (window.ada && window.ada.onTerminalData) {
        window.ada.onTerminalData((_event, data) => {
          term.write(data);
        });
      }

      term.onData((data) => {
        if (onData) onData(data);
        // In a real scenario, this would send data to the sandbox process
        // For now, we simulate a simple echo for local commands if not in an active process
      });

      term.writeln('\x1b[1;36mADA Isolated Sandbox Terminal v1.0\x1b[0m');
      term.writeln('Connected to: \x1b[1;33mdocker://ubuntu-jammy-rust-v1\x1b[0m');
      term.writeln('Type \x1b[1;32mhelp\x1b[0m to list available commands.');
      term.write('\n\r\x1b[1;35mroot@ada-sandbox:~\x1b[0m# ');

      xtermRef.current = term;

      const handleResize = () => fitAddon.fit();
      window.addEventListener('resize', handleResize);

      return () => {
        window.removeEventListener('resize', handleResize);
        term.dispose();
      };
    }
  }, []);

  return (
    <div className="flex-1 bg-black/40 p-4 font-mono overflow-hidden">
      <div ref={terminalRef} className="h-full w-full" />
    </div>
  );
};

export default TerminalView;
