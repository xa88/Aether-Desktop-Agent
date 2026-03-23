import React, { useState, useEffect } from 'react';
import { Search, Puzzle, Download, CheckCircle2, Globe, ExternalLink, Loader2 } from 'lucide-react';
import { motion, AnimatePresence } from 'framer-motion';

const PluginExplorer = () => {
  const [query, setQuery] = useState('');
  const [results, setResults] = useState([]);
  const [loading, setLoading] = useState(false);
  const [installing, setInstalling] = useState({}); // { id: boolean }

  const handleSearch = async (e) => {
    e?.preventDefault();
    if (!query.trim()) return;
    setLoading(true);
    try {
      const data = await window.ada.searchPlugins(query);
      setResults(data);
    } catch (err) {
      console.error("Search failed", err);
    } finally {
      setLoading(false);
    }
  };

  const handleInstall = async (plugin) => {
    setInstalling(prev => ({ ...prev, [plugin.id]: true }));
    try {
      const result = await window.ada.installPlugin(plugin);
      if (result.success) {
        setInstalling(prev => ({ ...prev, [plugin.id]: 'done' }));
      }
    } catch (err) {
      console.error("Install failed", err);
      setInstalling(prev => ({ ...prev, [plugin.id]: false }));
    }
  };

  useEffect(() => {
    const delayDebounceFn = setTimeout(() => {
      if (query) handleSearch();
    }, 500);
    return () => clearTimeout(delayDebounceFn);
  }, [query]);

  return (
    <div className="p-8 h-full bg-aether-space flex flex-col overflow-hidden">
      <header className="mb-10">
        <div className="flex items-center gap-4 mb-2">
          <Globe className="text-aether-indigo" size={32} />
          <h2 className="text-2xl font-bold tracking-tight">Open VSX Marketplace</h2>
        </div>
        <p className="text-sm text-white/40">Extend ADA with thousands of VSCode-compatible extensions.</p>
      </header>

      <form onSubmit={handleSearch} className="relative mb-8">
        <Search className="absolute left-4 top-1/2 -translate-y-1/2 text-white/20" size={18} />
        <input 
          type="text"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder="Search extensions (e.g. 'python', 'git', 'theme')..."
          className="w-full bg-black/40 border border-white/5 rounded-2xl pl-12 pr-4 py-4 outline-none focus:border-aether-indigo/40 transition-all text-white/80"
        />
      </form>

      <div className="flex-1 overflow-y-auto custom-scrollbar pr-4 grid grid-cols-1 md:grid-cols-2 gap-6 pb-12">
        <AnimatePresence>
          {loading ? (
            <div className="col-span-full flex flex-col items-center justify-center py-20 opacity-20">
               <Loader2 className="animate-spin mb-4" size={40} />
               <span className="text-sm font-bold uppercase tracking-widest">Querying Registry...</span>
            </div>
          ) : results.length > 0 ? (
            results.map((plugin, idx) => (
              <motion.div 
                key={plugin.id}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: idx * 0.05 }}
                className="p-6 rounded-3xl bg-white/5 border border-white/5 hover:border-white/10 transition-all group relative overflow-hidden"
              >
                <div className="flex items-start gap-4 mb-4">
                  <img src={plugin.icon} alt={plugin.name} className="w-12 h-12 rounded-xl bg-black/40" />
                  <div className="flex-1 min-w-0">
                    <h4 className="font-bold text-white/90 leading-tight mb-1 truncate">{plugin.name}</h4>
                    <p className="text-[10px] text-white/20 uppercase tracking-widest font-bold mb-2">by {plugin.publisher}</p>
                  </div>
                </div>

                <p className="text-xs text-white/50 mb-6 line-clamp-2 min-h-[32px]">{plugin.description}</p>

                <div className="flex items-center justify-between mt-auto">
                   <div className="flex gap-2">
                     <span className="px-2 py-0.5 rounded-full bg-white/5 border border-white/10 text-[8px] font-bold text-white/40 uppercase">v{plugin.version}</span>
                   </div>
                   
                   {installing[plugin.id] === 'done' ? (
                     <div className="flex items-center gap-2 text-green-500 text-[10px] font-bold uppercase tracking-widest">
                       <CheckCircle2 size={14} /> Installed
                     </div>
                   ) : (
                     <button 
                       onClick={() => handleInstall(plugin)}
                       disabled={installing[plugin.id]}
                       className="flex items-center gap-2 px-4 py-2 rounded-xl bg-aether-indigo/10 border border-aether-indigo/20 text-[10px] font-bold text-aether-indigo hover:bg-aether-indigo/20 transition-all uppercase tracking-widest disabled:opacity-50"
                     >
                       {installing[plugin.id] ? <Loader2 className="animate-spin" size={14} /> : <Download size={14} />}
                       {installing[plugin.id] ? 'Installing...' : 'Install'}
                     </button>
                   )}
                </div>
              </motion.div>
            ))
          ) : query && (
            <div className="col-span-full text-center py-20 text-white/20">
              No extensions found for "{query}"
            </div>
          )}
        </AnimatePresence>
      </div>
    </div>
  );
};

export default PluginExplorer;
