import React, { useState, useEffect } from 'react';
import { Search, Command, FileText, Settings, Code, Zap, ChevronRight } from 'lucide-react';
import { motion, AnimatePresence } from 'framer-motion';

const CommandPalette = ({ isOpen, onClose, onSelect }) => {
  const [query, setQuery] = useState('');
  
  const items = [
    { id: 'settings', label: 'Open Settings', icon: <Settings size={14} />, category: 'Action' },
    { id: 'new-workflow', label: 'Create New Playbook', icon: <Zap size={14} />, category: 'Action' },
    { id: 'scan-fs', label: 'Scan Projects Directory', icon: <Search size={14} />, category: 'Action' },
    { id: 'readme', label: 'README.md', icon: <FileText size={14} />, category: 'Files' },
    { id: 'main-rs', label: 'src/main.rs', icon: <Code size={14} />, category: 'Files' },
  ];

  const filteredItems = items.filter(i => i.label.toLowerCase().includes(query.toLowerCase()));

  useEffect(() => {
    const handleKeyDown = (e) => {
      if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
        e.preventDefault();
        onClose(); // Toggle
      }
      if (e.key === 'Escape') onClose();
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, []);

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-[100] flex items-start justify-center pt-[15vh] px-4 pointer-events-none">
       <div className="absolute inset-0 bg-black/60 backdrop-blur-sm pointer-events-auto" onClick={onClose} />
       
       <motion.div 
         initial={{ opacity: 0, scale: 0.95, y: -10 }}
         animate={{ opacity: 1, scale: 1, y: 0 }}
         className="w-full max-w-xl bg-aether-surface border border-white/10 rounded-2xl shadow-2xl overflow-hidden pointer-events-auto relative"
       >
          <div className="flex items-center px-4 py-4 border-b border-white/5 bg-white/5">
             <Search className="text-white/20 mr-3" size={18} />
             <input 
               autoFocus
               type="text"
               value={query}
               onChange={(e) => setQuery(e.target.value)}
               placeholder="Search ADA functions, files, or agents..."
               className="bg-transparent border-none w-full focus:ring-0 text-white placeholder:text-white/10 text-sm"
             />
             <div className="flex items-center gap-1.5 px-2 py-1 rounded bg-black/40 border border-white/10 text-[10px] text-white/40 font-mono">
                <Command size={10} /> K
             </div>
          </div>

          <div className="p-2 max-h-[60vh] overflow-y-auto custom-scrollbar">
             {filteredItems.length > 0 ? (
                <div className="space-y-4 py-2">
                   {/* Grouped logic could go here */}
                   {filteredItems.map((item, i) => (
                      <button 
                        key={item.id}
                        onClick={() => { onSelect(item.id); onClose(); }}
                        className="w-full flex items-center justify-between p-3 rounded-xl hover:bg-white/5 group transition-colors text-left"
                      >
                         <div className="flex items-center gap-3">
                            <div className="w-8 h-8 rounded-lg bg-black/40 flex items-center justify-center text-white/20 group-hover:text-aether-indigo transition-colors">
                               {item.icon}
                            </div>
                            <span className="text-sm font-medium text-white/60 group-hover:text-white transition-colors">{item.label}</span>
                         </div>
                         <div className="flex items-center gap-2">
                            <span className="text-[10px] uppercase tracking-widest text-white/20 font-bold group-hover:text-white/40">{item.category}</span>
                            <ChevronRight size={14} className="text-white/10" />
                         </div>
                      </button>
                   ))}
                </div>
             ) : (
                <div className="py-12 text-center text-white/20">
                   <p className="text-sm italic">No results found for "{query}"</p>
                </div>
             )}
          </div>

          <div className="px-4 py-2 border-t border-white/5 bg-black/20 flex justify-between items-center bg-white/5">
             <div className="flex items-center gap-3">
                <div className="flex items-center gap-1 text-[10px] text-white/20"><span className="px-1.5 py-0.5 rounded bg-white/5 font-mono">↑↓</span> to navigate</div>
                <div className="flex items-center gap-1 text-[10px] text-white/20"><span className="px-1.5 py-0.5 rounded bg-white/5 font-mono">enter</span> to select</div>
             </div>
             <div className="text-[10px] text-white/20 font-medium tracking-tight">Aether Navigation Engine</div>
          </div>
       </motion.div>
    </div>
  );
};

export default CommandPalette;
