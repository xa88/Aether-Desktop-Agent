import React, { useState } from 'react';
import { ToggleLeft, ToggleRight, Puzzle, Plus, Search, ShieldCheck, AlertTriangle, ExternalLink } from 'lucide-react';
import { motion } from 'framer-motion';

const PluginManager = () => {
  const [plugins, setPlugins] = useState([]);
  const [searchQuery, setSearchQuery] = useState('');
  const [activeTab, setActiveTab] = useState('installed');
  const [isInstalling, setIsInstalling] = useState(null);

  useEffect(() => {
    const fetchPlugins = async () => {
      const results = await window.ada.searchPlugins('');
      setPlugins(results.map(p => ({ ...p, enabled: true, security: p.source === 'Internal' ? 'verified' : 'unverified' })));
    };
    fetchPlugins();
  }, []);

  const handleSearch = async (e) => {
    setSearchQuery(e.target.value);
    const results = await window.ada.searchPlugins(e.target.value);
    setPlugins(results.map(p => ({ ...p, enabled: true, security: p.source === 'Internal' ? 'verified' : 'unverified' })));
  };

  const handleInstall = async (id) => {
    setIsInstalling(id);
    await window.ada.installPlugin(id);
    setIsInstalling(null);
    // Refresh or show success
  };

  const togglePlugin = (id) => {
    setPlugins(prev => prev.map(p => p.id === id ? { ...p, enabled: !p.enabled } : p));
  };

  return (
    <div className="flex h-full bg-aether-space overflow-hidden">
      <div className="flex-1 flex flex-col">
        <header className="h-16 border-b border-white/5 flex items-center justify-between px-8 bg-black/20">
           <div className="flex items-center gap-4 flex-1">
              <h2 className="text-xl font-bold mr-8">Extensions</h2>
              <div className="flex items-center gap-4">
                <button 
                  onClick={() => setActiveTab('installed')} 
                  className={`text-sm font-bold py-2 px-4 rounded-lg transition-colors ${activeTab === 'installed' ? 'bg-aether-indigo text-white' : 'text-white/50 hover:text-white/70'}`}
                >
                  Installed
                </button>
                <button 
                  onClick={() => setActiveTab('marketplace')} 
                  className={`text-sm font-bold py-2 px-4 rounded-lg transition-colors ${activeTab === 'marketplace' ? 'bg-aether-indigo text-white' : 'text-white/50 hover:text-white/70'}`}
                >
                  Marketplace
                </button>
              </div>
              <div className="relative w-full max-w-md ml-8">
                 <Search className="absolute left-3 top-1/2 -translate-y-1/2 text-white/20" size={14} />
                 <input 
                   type="text" 
                   value={searchQuery}
                   onChange={handleSearch}
                   placeholder="Search Open VSX / Internal Marketplace..."
                   className="w-full bg-white/5 border border-white/10 rounded-xl pl-10 pr-4 py-2 text-xs focus:border-aether-indigo outline-none transition-colors"
                 />
              </div>
           </div>
           <button className="flex items-center gap-2 px-4 py-2 bg-aether-indigo hover:bg-aether-indigo/80 rounded-xl text-xs font-bold transition-all shadow-lg shadow-aether-indigo/20">
             <Plus size={16} />
             Install Plugin
           </button>
        </header>

        <div className="flex-1 overflow-y-auto p-8 custom-scrollbar">
           <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
              {plugins.map((p, i) => (
                <motion.div 
                  key={p.id}
                  initial={{ opacity: 0, scale: 0.95 }}
                  animate={{ opacity: 1, scale: 1 }}
                  transition={{ delay: i * 0.05 }}
                  className={`p-6 rounded-2xl border transition-all ${p.enabled ? 'bg-white/5 border-white/10' : 'bg-black/20 border-white/5 grayscale opacity-60'}`}
                >
                  <div className="flex items-start justify-between mb-4">
                    <div className="flex gap-4">
                      <div className={`w-12 h-12 rounded-xl flex items-center justify-center ${p.enabled ? 'bg-aether-indigo/20 text-aether-indigo' : 'bg-white/5 text-white/20'}`}>
                        <Puzzle size={24} />
                      </div>
                       <div>
                        <h3 className="font-bold text-white/80">{p.name}</h3>
                        <p className="text-[10px] text-white/40">v{p.version} • {p.source || 'Community'}</p>
                      </div>
                    </div>
                    <div className="flex flex-col items-end gap-2">
                       {activeTab === 'installed' && (
                         <button onClick={() => togglePlugin(p.id)} className="transition-transform hover:scale-110">
                           {p.enabled ? <ToggleRight size={28} className="text-aether-indigo" /> : <ToggleLeft size={28} className="text-white/20" />}
                         </button>
                       )}
                       {activeTab === 'marketplace' && !p.installed && (
                         <button 
                           onClick={() => handleInstall(p.id)}
                           disabled={isInstalling === p.id}
                           className={`text-[8px] font-bold uppercase tracking-wider px-2 py-1 rounded border ${isInstalling === p.id ? 'opacity-50' : 'hover:bg-white/5'}`}
                         >
                           {isInstalling === p.id ? 'Installing...' : 'Install'}
                         </button>
                       )}
                       {activeTab === 'marketplace' && p.installed && (
                         <span className="text-[8px] font-bold uppercase tracking-wider px-2 py-1 rounded border border-green-500 text-green-500">Installed</span>
                       )}
                    </div>
                  </div>
                  
                  <p className="text-xs text-white/60 mb-6 leading-relaxed h-8 line-clamp-2">
                    {p.desc}
                  </p>

                  <div className="flex items-center justify-between border-t border-white/5 pt-4">
                    <div className="flex items-center gap-2">
                       {p.security === 'verified' ? (
                         <div className="flex items-center gap-1.5 px-2 py-0.5 rounded-full bg-green-500/10 text-green-500 text-[10px] font-bold">
                           <ShieldCheck size={10} /> Verified
                         </div>
                       ) : (
                         <div className="flex items-center gap-1.5 px-2 py-0.5 rounded-full bg-yellow-500/10 text-yellow-500 text-[10px] font-bold">
                           <AlertTriangle size={10} /> Experimental
                         </div>
                       )}
                    </div>
                    <div className="flex flex-wrap gap-2">
                       {p.permissions?.map(perm => (
                         <span key={perm} className="px-1.5 py-0.5 rounded bg-white/5 text-[8px] text-white/40 font-mono border border-white/5">{perm}</span>
                       ))}
                    </div>
                    <button className="text-[10px] font-bold text-white/30 hover:text-white flex items-center gap-1 transition-colors uppercase tracking-wider">
                      More <ExternalLink size={10} />
                    </button>
                  </div>
                </motion.div>
              ))}
           </div>
        </div>
      </div>
    </div>
  );
};

export default PluginManager;
