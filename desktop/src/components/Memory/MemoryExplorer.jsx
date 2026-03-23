import React, { useState, useEffect } from 'react';
import { BrainCircuit, History, Search, Zap, Trash2, ExternalLink } from 'lucide-react';
import { motion } from 'framer-motion';

const MemoryExplorer = () => {
  const [experiences, setExperiences] = useState([]);
  const [searchTerm, setSearchTerm] = useState('');

  useEffect(() => {
    const fetchMemory = async () => {
      if (window.ada) {
        const data = await window.ada.getStoredExperiences();
        setExperiences(data);
      }
    };
    fetchMemory();
  }, []);

  const filtered = experiences.filter(exp => 
    exp.signature.toLowerCase().includes(searchTerm.toLowerCase())
  );

  return (
    <div className="p-8 h-full bg-aether-space flex flex-col overflow-hidden">
      <header className="mb-10">
        <div className="flex items-center gap-4 mb-2">
          <BrainCircuit className="text-aether-indigo" size={32} />
          <h2 className="text-2xl font-bold tracking-tight">Perceptual Memory Explorer</h2>
        </div>
        <p className="text-sm text-white/40">Long-term semantic recall of successful autonomous operations.</p>
      </header>

      <div className="relative mb-8">
        <Search className="absolute left-4 top-1/2 -translate-y-1/2 text-white/20" size={18} />
        <input 
          type="text"
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
          placeholder="Search semantic history..."
          className="w-full bg-black/40 border border-white/5 rounded-2xl pl-12 pr-4 py-4 outline-none focus:border-aether-indigo/40 transition-all text-white/80"
        />
      </div>

      <div className="flex-1 overflow-y-auto custom-scrollbar pr-4 grid grid-cols-1 md:grid-cols-2 gap-6 pb-12">
        {filtered.map((exp, idx) => (
          <motion.div 
            key={idx}
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: idx * 0.05 }}
            className="p-6 rounded-3xl bg-white/5 border border-white/5 hover:border-white/10 transition-all group relative overflow-hidden"
          >
            <div className="absolute top-0 right-0 p-4 opacity-0 group-hover:opacity-100 transition-opacity">
              <ExternalLink size={14} className="text-white/40 cursor-pointer hover:text-aether-indigo" />
            </div>

            <div className="flex items-start gap-4 mb-6">
              <div className="w-10 h-10 rounded-xl bg-aether-indigo/10 flex items-center justify-center">
                <History size={20} className="text-aether-indigo" />
              </div>
              <div>
                <h4 className="font-bold text-white/90 leading-tight mb-1">{exp.signature}</h4>
                <div className="flex items-center gap-3 text-[10px] text-white/20 uppercase tracking-widest font-bold">
                  <span>{new Date(exp.timestamp).toLocaleDateString()}</span>
                  <span className="w-1 h-1 rounded-full bg-white/10" />
                  <span>{exp.model}</span>
                </div>
              </div>
            </div>

            <div className="flex gap-2">
               <span className="px-2 py-0.5 rounded-full bg-green-500/10 border border-green-500/20 text-[8px] font-bold text-green-500 uppercase">Verified Success</span>
               <span className="px-2 py-0.5 rounded-full bg-white/5 border border-white/10 text-[8px] font-bold text-white/40 uppercase">Recall Potential: 98%</span>
            </div>
            
            <div className="mt-6 flex gap-3">
              <button className="flex-1 py-2 rounded-xl bg-white/5 border border-white/10 text-[10px] font-bold hover:bg-white/10 transition-colors uppercase tracking-widest">View Fragment</button>
              <button className="px-4 py-2 rounded-xl bg-white/5 border border-white/10 text-white/20 hover:text-red-400 hover:border-red-400/20 transition-all"><Trash2 size={14} /></button>
            </div>
          </motion.div>
        ))}
      </div>
    </div>
  );
};

export default MemoryExplorer;
