import React, { useState } from 'react';
import { Image as ImageIcon, Search, Filter, Maximize2, Tag, Calendar, Cpu } from 'lucide-react';
import { motion } from 'framer-motion';

const VisionGallery = () => {
  const [selectedImage, setSelectedImage] = useState(null);

  // Mock vision captures
  const captures = [
    { id: 1, name: 'desktop_scan_01.png', ts: '2 min ago', tags: ['Desktop', 'UI', 'Chrome'], size: '1.2MB' },
    { id: 2, name: 'error_dialog_crash.png', ts: '15 min ago', tags: ['Error', 'Dialog', 'System'], size: '450KB' },
    { id: 3, name: 'code_snippet_ref.jpg', ts: '1 hour ago', tags: ['IDE', 'Code', 'Python'], size: '890KB' },
    { id: 4, name: 'web_search_context.png', ts: '3 hours ago', tags: ['Search', 'Google', 'Documentation'], size: '2.1MB' },
  ];

  return (
    <div className="flex h-full bg-aether-space">
      {/* Sidebar Filters */}
      <div className="w-64 border-r border-white/5 flex flex-col p-6 bg-black/20">
        <div className="text-[10px] font-bold text-white/30 uppercase tracking-[0.2em] mb-6">Vision Hub</div>
        
        <div className="space-y-6">
          <div className="space-y-3">
            <h4 className="text-xs font-semibold text-white/60 flex items-center gap-2">
              <Filter size={14} /> Filters
            </h4>
            <div className="space-y-1">
              {['All Captures', 'Screenshots', 'Object Assets', 'Redacted'].map(f => (
                <button key={f} className={`w-full text-left text-xs px-2 py-1.5 rounded-lg transition-colors ${f === 'All Captures' ? 'bg-aether-indigo/10 text-aether-indigo' : 'text-white/40 hover:text-white hover:bg-white/5'}`}>
                  {f}
                </button>
              ))}
            </div>
          </div>

          <div className="space-y-3">
            <h4 className="text-xs font-semibold text-white/60 flex items-center gap-2">
              <Tag size={14} /> Popular Tags
            </h4>
            <div className="flex flex-wrap gap-2">
              {['System', 'UI', 'Error', 'Code', 'Web'].map(t => (
                <span key={t} className="px-2 py-0.5 rounded bg-white/5 border border-white/5 text-[10px] text-white/40 cursor-pointer hover:border-aether-indigo/30 hover:text-white transition-all">
                  {t}
                </span>
              ))}
            </div>
          </div>
        </div>

        <div className="mt-auto p-4 rounded-xl bg-gradient-to-br from-aether-indigo/10 to-transparent border border-aether-indigo/20">
           <div className="flex items-center gap-2 mb-2">
             <Cpu size={14} className="text-aether-indigo" />
             <span className="text-[10px] font-bold text-white/80 uppercase tracking-widest">OCR Pipeline</span>
           </div>
           <p className="text-[10px] text-white/40">Real-time text extraction and object labeling active.</p>
        </div>
      </div>

      {/* Main Gallery */}
      <div className="flex-1 flex flex-col">
        <header className="h-16 border-b border-white/5 flex items-center justify-between px-8 bg-black/20">
           <div className="flex items-center gap-4 flex-1">
              <div className="relative w-full max-w-md">
                 <Search className="absolute left-3 top-1/2 -translate-y-1/2 text-white/20" size={14} />
                 <input 
                   type="text" 
                   placeholder="Search captures by tag, content or text..."
                   className="w-full bg-white/5 border border-white/10 rounded-xl pl-10 pr-4 py-2 text-xs focus:border-aether-indigo outline-none transition-colors placeholder:text-white/10"
                 />
              </div>
           </div>
           <div className="flex items-center gap-4 ml-8">
              <button className="text-xs font-bold text-white/30 hover:text-white uppercase tracking-widest transition-colors">Clear Gallery</button>
           </div>
        </header>

        <div className="flex-1 overflow-y-auto p-8 custom-scrollbar">
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
            {captures.map((c, i) => (
              <motion.div 
                key={c.id}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: i * 0.05 }}
                className="group cursor-pointer"
                onClick={() => setSelectedImage(c)}
              >
                <div className="aspect-video bg-white/5 border border-white/10 rounded-2xl overflow-hidden relative mb-3 hover:border-aether-indigo/50 transition-colors shadow-lg hover:shadow-aether-indigo/10">
                   <div className="absolute inset-0 flex items-center justify-center">
                     <ImageIcon size={32} className="text-white/10 group-hover:scale-110 transition-transform duration-500" />
                   </div>
                   <div className="absolute inset-x-0 bottom-0 p-3 bg-gradient-to-t from-black/80 to-transparent opacity-0 group-hover:opacity-100 transition-opacity">
                      <div className="flex items-center justify-between">
                         <span className="text-[10px] text-white/60 font-mono">{c.size}</span>
                         <Maximize2 size={12} className="text-white/80" />
                      </div>
                   </div>
                </div>
                <div className="px-1">
                  <h5 className="text-[11px] font-bold text-white/80 truncate mb-1">{c.name}</h5>
                  <div className="flex items-center justify-between">
                     <span className="text-[10px] text-white/30 flex items-center gap-1">
                        <Calendar size={10} /> {c.ts}
                     </span>
                     <div className="flex gap-1">
                        {c.tags.slice(0, 1).map(t => (
                          <span key={t} className="text-[8px] px-1.5 py-0.5 rounded bg-aether-indigo/20 text-aether-indigo uppercase font-bold tracking-wider">{t}</span>
                        ))}
                     </div>
                  </div>
                </div>
              </motion.div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
};

export default VisionGallery;
