import React, { useState } from 'react';
import { Box, FileJson, Download, Share2, Info, Search, Filter, History, CheckCircle2 } from 'lucide-react';
import { motion } from 'framer-motion';

const ArtifactExplorer = () => {
  const [bundles, setBundles] = useState([
    { id: 'b1', name: 'Refactor_Core_Logic_Evidence', date: '2026-03-23', size: '4.2MB', hash: 'sha256:7a1b...8E', steps: 12, status: 'verified' },
    { id: 'b2', name: 'Fix_Memory_Leak_Bundle', date: '2026-03-22', size: '1.8MB', hash: 'sha256:f9c2...01', steps: 5, status: 'verified' },
    { id: 'b3', name: 'Security_Audit_Report', date: '2026-03-21', size: '890KB', hash: 'sha256:3d2e...A9', steps: 8, status: 'verified' },
  ]);

  return (
    <div className="flex h-full bg-aether-space flex-col">
      <header className="h-16 border-b border-white/5 flex items-center justify-between px-8 bg-black/20">
         <div className="flex items-center gap-4 flex-1">
            <h2 className="text-xl font-bold mr-8">Artifact Explorer</h2>
            <div className="relative w-full max-w-md">
               <Search className="absolute left-3 top-1/2 -translate-y-1/2 text-white/20" size={14} />
               <input 
                 type="text" 
                 placeholder="Search by bundle ID, hash or project name..."
                 className="w-full bg-white/5 border border-white/10 rounded-xl pl-10 pr-4 py-2 text-xs focus:border-aether-indigo outline-none transition-colors"
               />
            </div>
         </div>
         <div className="flex items-center gap-3">
            <button className="p-2 hover:bg-white/5 rounded-lg text-white/40 transition-colors"><Filter size={18} /></button>
            <button className="p-2 hover:bg-white/5 rounded-lg text-white/40 transition-colors"><History size={18} /></button>
         </div>
      </header>

      <div className="flex-1 overflow-y-auto p-8 custom-scrollbar">
         <div className="grid grid-cols-1 gap-4">
            {bundles.map((bundle, i) => (
              <motion.div 
                key={bundle.id}
                initial={{ opacity: 0, y: 10 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: i * 0.05 }}
                className="group flex items-center gap-6 p-6 rounded-2xl bg-white/5 border border-white/5 hover:border-white/10 transition-all cursor-pointer"
              >
                <div className="w-12 h-12 rounded-xl bg-aether-indigo/10 flex items-center justify-center text-aether-indigo group-hover:bg-aether-indigo group-hover:text-white transition-all">
                   <Box size={24} />
                </div>
                
                <div className="flex-1 min-w-0">
                   <div className="flex items-center gap-3 mb-1">
                      <h3 className="font-bold text-white/80 truncate">{bundle.name}</h3>
                      <div className="flex items-center gap-1.5 px-2 py-0.5 rounded-full bg-green-500/10 text-green-500 text-[9px] font-bold uppercase tracking-wider">
                         <CheckCircle2 size={10} /> {bundle.status}
                      </div>
                   </div>
                   <p className="text-[10px] font-mono text-white/30 truncate">
                      {bundle.hash} • {bundle.date} • {bundle.size} • {bundle.steps} Steps
                   </p>
                </div>

                <div className="flex items-center gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
                   <button className="p-2 rounded-lg bg-white/5 hover:bg-white/10 text-white/60 transition-colors" title="Download Bundle"><Download size={16} /></button>
                   <button className="p-2 rounded-lg bg-white/5 hover:bg-white/10 text-white/60 transition-colors" title="Share Manifest"><Share2 size={16} /></button>
                   <button className="p-2 rounded-lg bg-white/5 hover:bg-white/10 text-white/60 transition-colors" title="View Details"><Info size={16} /></button>
                </div>
              </motion.div>
            ))}
         </div>

         {/* Bottom Action Area */}
         <div className="mt-12 p-8 rounded-3xl bg-gradient-to-br from-aether-indigo/10 to-transparent border border-aether-indigo/20 flex items-center justify-between">
            <div className="flex items-center gap-6">
               <div className="w-16 h-16 rounded-2xl bg-black/40 border border-white/5 flex items-center justify-center text-aether-indigo">
                  <FileJson size={32} />
               </div>
               <div>
                  <h4 className="text-lg font-bold text-white/90 mb-1">Export Evidence Package</h4>
                  <p className="text-xs text-white/40">Generate a signed bundle containing logs, plan hierarchy, and system fingerprints for reproducibility.</p>
               </div>
            </div>
            <button className="px-6 py-3 bg-aether-indigo hover:bg-aether-indigo/80 rounded-2xl text-xs font-bold transition-all shadow-lg shadow-aether-indigo/30">
               Generate Manifest
            </button>
         </div>
      </div>
    </div>
  );
};

export default ArtifactExplorer;
