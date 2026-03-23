import React, { useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { ChevronRight, Mic, Globe, Shield, Cpu, Save, X, Plus } from 'lucide-react';

const SettingsPanel = ({ isOpen, onClose, onSave }) => {
  const [directorModel, setDirectorModel] = useState('gpt-4o');
  const [workerModel, setWorkerModel] = useState('nemotron-3-8b');
  const [baseUrl, setBaseUrl] = useState('https://api.openai.com/v1');
  const [apiKey, setApiKey] = useState('');
  const [testStatus, setTestStatus] = useState({}); // { director: 'idle' | 'testing' | 'valid' | 'error' }

  const handleTest = async (type) => {
    const model = type === 'director' ? directorModel : workerModel;
    setTestStatus(prev => ({ ...prev, [type]: 'testing' }));
    
    try {
      const result = await window.ada.testLlmConnection({ baseUrl, apiKey, model });
      if (result.success) {
        setTestStatus(prev => ({ ...prev, [type]: 'valid' }));
      } else {
        setTestStatus(prev => ({ ...prev, [type]: 'error' }));
      }
    } catch (err) {
      setTestStatus(prev => ({ ...prev, [type]: 'error' }));
    }
  };

  const handleSave = () => {
    onSave({ directorModel, workerModel, baseUrl, apiKey });
  };
  return (
    <AnimatePresence>
      {isOpen && (
        <>
          <motion.div 
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            className="fixed inset-0 bg-black/60 backdrop-blur-sm z-40"
            onClick={onClose}
          />
          <motion.div 
            initial={{ x: '100%' }}
            animate={{ x: 0 }}
            exit={{ x: '100%' }}
            transition={{ type: 'spring', damping: 25, stiffness: 200 }}
            className="fixed right-0 top-0 h-full w-full max-w-md bg-aether-surface border-l border-white/10 z-50 shadow-2xl p-8 flex flex-col"
          >
            <div className="flex items-center justify-between mb-8">
              <h2 className="text-xl font-bold">System Configuration</h2>
              <button 
                onClick={onClose}
                className="p-2 hover:bg-white/5 rounded-full transition-colors text-white/40 hover:text-white"
              >
                <X size={20} />
              </button>
            </div>

            <div className="space-y-8 flex-1 overflow-y-auto custom-scrollbar pr-2">
              <section>
                <div className="flex items-center gap-2 mb-4 text-aether-indigo">
                   <Cpu size={14} />
                   <h3 className="text-[10px] font-bold uppercase tracking-widest">Autonomous Orchestration</h3>
                </div>
                <div className="space-y-4">
                  <div className="p-4 rounded-2xl bg-white/5 border border-white/5 space-y-4">
                    <div className="space-y-2">
                      <div className="flex justify-between items-center">
                        <label className="text-[10px] font-bold text-white/40 uppercase tracking-widest ml-1">Director Model (Planning)</label>
                        <div className="flex items-center gap-2">
                          {testStatus.director === 'testing' && <motion.div animate={{ rotate: 360 }} transition={{ repeat: Infinity, duration: 1 }} className="text-aether-indigo"><div className="w-2 h-2 border-2 border-aether-indigo border-t-transparent rounded-full" /></motion.div>}
                          {testStatus.director === 'valid' && <span className="text-green-500 text-[8px] font-bold">READY</span>}
                          {testStatus.director === 'error' && <span className="text-red-500 text-[8px] font-bold">FAIL</span>}
                          <button onClick={() => handleTest('director')} className="text-[8px] text-aether-indigo/60 hover:text-aether-indigo px-2 py-0.5 rounded bg-white/5 border border-white/10 transition-all font-bold uppercase tracking-tighter">Test</button>
                        </div>
                      </div>
                      <input 
                        type="text" 
                        value={directorModel}
                        onChange={(e) => setDirectorModel(e.target.value)}
                        className="w-full bg-black/30 border border-white/10 rounded-xl px-4 py-3 text-sm focus:border-aether-indigo outline-none transition-colors placeholder:text-white/10 text-white/80" 
                        placeholder="e.g. gpt-4o" 
                      />
                    </div>
                    <div className="space-y-2">
                      <div className="flex justify-between items-center">
                        <label className="text-[10_px] font-bold text-white/40 uppercase tracking-widest ml-1">Worker Model (Execution)</label>
                        <div className="flex items-center gap-2">
                          {testStatus.worker === 'testing' && <motion.div animate={{ rotate: 360 }} transition={{ repeat: Infinity, duration: 1 }} className="text-green-500"><div className="w-2 h-2 border-2 border-green-500 border-t-transparent rounded-full" /></motion.div>}
                          {testStatus.worker === 'valid' && <span className="text-green-500 text-[8px] font-bold">READY</span>}
                          {testStatus.worker === 'error' && <span className="text-red-500 text-[8px] font-bold">FAIL</span>}
                          <button onClick={() => handleTest('worker')} className="text-[8px] text-green-500/60 hover:text-green-500 px-2 py-0.5 rounded bg-white/5 border border-white/10 transition-all font-bold uppercase tracking-tighter">Test</button>
                        </div>
                      </div>
                      <input 
                        type="text" 
                        value={workerModel}
                        onChange={(e) => setWorkerModel(e.target.value)}
                        className="w-full bg-black/30 border border-white/10 rounded-xl px-4 py-3 text-sm focus:border-aether-indigo outline-none transition-colors placeholder:text-white/10 text-white/80" 
                        placeholder="e.g. nemotron-3-8b" 
                      />
                    </div>
                  </div>
                </div>
              </section>

              <section>
                <div className="flex items-center gap-2 mb-4 text-white/40">
                   <Globe size={14} />
                   <h3 className="text-[10px] font-bold uppercase tracking-widest">Network Gateway</h3>
                </div>
                <div className="space-y-4">
                  <div className="space-y-2">
                    <label className="text-[10px] font-bold text-white/40 uppercase tracking-widest ml-1">Endpoint URL</label>
                    <input 
                      type="text" 
                      value={baseUrl}
                      onChange={(e) => setBaseUrl(e.target.value)}
                      className="w-full bg-black/30 border border-white/10 rounded-xl px-4 py-3 text-sm focus:border-aether-indigo outline-none transition-colors placeholder:text-white/10 text-white/80" 
                      placeholder="https://api.openai.com/v1" 
                    />
                  </div>
                  <div className="space-y-2">
                    <label className="text-[10px] font-bold text-white/40 uppercase tracking-widest ml-1">Secret Key</label>
                    <input 
                      type="password" 
                      value={apiKey}
                      onChange={(e) => setApiKey(e.target.value)}
                      className="w-full bg-black/30 border border-white/10 rounded-xl px-4 py-3 text-sm focus:border-aether-indigo outline-none transition-colors placeholder:text-white/10 text-white/80" 
                      placeholder="••••••••••••••••••••••••" 
                    />
                  </div>
                </div>
              </section>

              <section>
                <div className="flex items-center gap-2 mb-4 text-green-500">
                   <Shield size={14} />
                   <h3 className="text-[10px] font-bold uppercase tracking-widest">Security & Privacy</h3>
                </div>
                <div className="space-y-3">
                  <ToggleItem icon={<Mic size={16} />} label="Voice Wake Word" active={true} />
                  <ToggleItem icon={<Globe size={16} />} label="Global Proxy (Tor/VPN)" active={false} />
                  <ToggleItem icon={<Shield size={16} />} label="Data Redaction (PII)" active={true} />
                  <ToggleItem icon={<Cpu size={16} />} label="Offline Mode (Local Only)" active={false} />
                  <div className="flex items-center justify-between p-4 rounded-2xl bg-white/5 border border-dashed border-white/10 hover:border-white/20 transition-all cursor-pointer group">
                     <span className="text-xs font-bold text-white/30 uppercase tracking-widest group-hover:text-white/60 transition-colors">Import manual_plan.yaml</span>
                     <Plus size={14} className="text-white/20" />
                  </div>
                </div>
              </section>

              <section>
                <h3 className="text-[10px] font-bold text-white/20 uppercase tracking-widest ml-1 mb-4">Aether Instance Info</h3>
                <div className="p-4 rounded-2xl bg-white/5 border border-white/5 space-y-3">
                   <div className="flex justify-between text-[11px]"><span className="text-white/30">Node ID</span> <span className="text-white/60 font-mono">ADA-992-QX</span></div>
                   <div className="flex justify-between text-[11px]"><span className="text-white/30">Release</span> <span className="text-white/60 font-mono">1.0.4-STABLE</span></div>
                   <div className="flex justify-between text-[11px]"><span className="text-white/30">Kernel</span> <span className="text-white/60 font-mono">v0.22.1-rust</span></div>
                </div>
              </section>
            </div>

            <div className="pt-6 mt-auto border-t border-white/5">
              <button 
                onClick={handleSave}
                className="w-full bg-aether-indigo hover:bg-aether-indigo/80 text-white font-bold py-4 rounded-2xl transition-all shadow-lg shadow-aether-indigo/30 flex items-center justify-center gap-2"
              >
                <Save size={18} />
                Commit Transitions
              </button>
            </div>
          </motion.div>
        </>
      )}
    </AnimatePresence>
  );
};

const ToggleItem = ({ icon, label, active }) => (
  <div className="flex items-center justify-between p-4 rounded-2xl bg-white/5 border border-white/5 hover:border-white/10 transition-colors">
    <div className="flex items-center gap-4">
      <div className="text-white/40">{icon}</div>
      <span className="text-sm font-medium text-white/80">{label}</span>
    </div>
    <div className={`w-10 h-5 rounded-full relative p-1 cursor-pointer transition-colors ${active ? 'bg-aether-indigo' : 'bg-white/10'}`}>
      <div className={`w-3 h-3 bg-white rounded-full transition-all ${active ? 'ml-auto' : 'ml-0'}`} />
    </div>
  </div>
);

export default SettingsPanel;
