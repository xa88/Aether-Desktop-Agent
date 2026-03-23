import React, { useState } from 'react';
import { Shield, Key, Lock, Eye, EyeOff, Trash2, Plus, ShieldAlert, Cpu, Database } from 'lucide-react';
import { motion } from 'framer-motion';

const IdentityVault = () => {
  const [secrets, setSecrets] = useState([
    { id: 1, name: 'OpenAI API Key', type: 'Env Var', provider: 'Global', redacted: true },
    { id: 2, name: 'AWS Production Access', type: 'IAM Role', provider: 'Cloud', redacted: true },
    { id: 3, name: 'GitHub OAuth Token', type: 'Token', provider: 'Local', redacted: true },
    { id: 4, name: 'SSH Primary Key', type: 'File', provider: 'Host', redacted: true },
  ]);

  const handleStoreSecret = async () => {
    const key = prompt("Enter secret name (e.g. ADA_API_KEY):");
    const val = prompt("Enter secret value:");
    if (key && val) {
      try {
        await window.ada.vaultSetSecret(key, val);
        setSecrets([...secrets, { id: Date.now(), name: key, type: 'Manual', provider: 'Vault', redacted: true }]);
      } catch (e) {
        alert("Failed to store secret in system keyring.");
      }
    }
  };

  return (
    <div className="flex h-full bg-aether-space flex-col">
      <header className="h-16 border-b border-white/5 flex items-center justify-between px-8 bg-black/20">
         <div className="flex items-center gap-4">
            <h2 className="text-xl font-bold">Identity Vault</h2>
            <div className="flex items-center gap-1.5 px-2 py-0.5 rounded-full bg-green-500/10 text-green-500 text-[10px] font-bold uppercase tracking-wider border border-green-500/20">
               <Shield size={12} /> Encrypted
            </div>
         </div>
         <button 
            onClick={handleStoreSecret}
            className="flex items-center gap-2 px-4 py-2 bg-aether-indigo hover:bg-aether-indigo/80 rounded-xl text-xs font-bold transition-all shadow-lg shadow-aether-indigo/20"
         >
            <Plus size={16} />
            Store Secret
         </button>
      </header>

      <div className="flex-1 overflow-y-auto p-8 custom-scrollbar">
         <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-12">
            {secrets.map((secret, i) => (
              <motion.div 
                key={secret.id}
                initial={{ opacity: 0, scale: 0.95 }}
                animate={{ opacity: 1, scale: 1 }}
                transition={{ delay: i * 0.05 }}
                className="p-6 rounded-2xl bg-white/5 border border-white/5 hover:border-white/10 transition-all group"
              >
                <div className="flex items-start justify-between mb-6">
                   <div className="flex gap-4">
                      <div className="w-12 h-12 rounded-xl bg-black/40 flex items-center justify-center text-aether-indigo border border-white/5">
                         <Key size={24} />
                      </div>
                      <div>
                         <h3 className="font-bold text-white/80">{secret.name}</h3>
                         <p className="text-[10px] text-white/30 font-mono uppercase tracking-widest">{secret.type} • {secret.provider}</p>
                      </div>
                   </div>
                   <div className="flex gap-2">
                      <button className="p-2 rounded-lg bg-white/5 hover:bg-white/10 text-white/40 hover:text-white transition-colors">
                         <Eye size={16} />
                      </button>
                      <button className="p-2 rounded-lg bg-white/5 hover:bg-white/10 text-white/40 hover:text-red-500 transition-colors">
                         <Trash2 size={16} />
                      </button>
                   </div>
                </div>

                <div className="flex items-center gap-3 bg-black/40 p-3 rounded-xl border border-white/5 group-hover:border-aether-indigo/30 transition-colors">
                   <Lock size={12} className="text-white/20" />
                   <div className="flex-1 flex gap-1">
                      {[...Array(24)].map((_, i) => (
                        <div key={i} className="w-1 h-1 rounded-full bg-white/10" />
                      ))}
                   </div>
                   <div className="text-[8px] font-bold text-aether-indigo uppercase tracking-widest">AES-256</div>
                </div>
              </motion.div>
            ))}
         </div>

         <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
            <div className="lg:col-span-2 p-8 rounded-3xl bg-gradient-to-br from-red-500/5 to-transparent border border-red-500/10">
               <div className="flex items-start gap-4">
                  <div className="w-12 h-12 rounded-2xl bg-red-500/10 flex items-center justify-center text-red-500">
                     <ShieldAlert size={28} />
                  </div>
                  <div>
                     <h4 className="text-sm font-bold text-white/90 mb-2">Automated PII Redaction Active</h4>
                     <p className="text-xs text-white/40 leading-relaxed max-w-lg">
                        The Aether Host Guard is currently monitoring all outbound telemetry and logs. 
                        Detected API keys, passwords, and private identifiers are automatically scrubbed 
                        before leaving the secure enclave.
                     </p>
                  </div>
               </div>
            </div>
            
            <div className="p-8 rounded-3xl bg-white/5 border border-white/5 flex flex-col justify-between">
                <div>
                   <h4 className="text-[10px] font-bold text-white/30 uppercase tracking-[0.2em] mb-4">Hardened Identity</h4>
                   <div className="space-y-4">
                      <div className="flex justify-between items-center text-xs">
                         <span className="text-white/40 flex items-center gap-2"><Cpu size={14} /> TPM 2.0</span>
                         <span className="text-green-500 font-bold uppercase tracking-widest text-[10px]">Active</span>
                      </div>
                      <div className="flex justify-between items-center text-xs">
                         <span className="text-white/40 flex items-center gap-2"><Database size={14} /> Local Vault</span>
                         <span className="text-green-500 font-bold uppercase tracking-widest text-[10px]">Synced</span>
                      </div>
                   </div>
                </div>
                <button className="mt-8 text-[10px] font-bold text-aether-indigo hover:text-white uppercase tracking-[0.2em] transition-colors text-left flex items-center gap-2">
                   Audit Access Logs <ChevronRight size={12} />
                </button>
            </div>
         </div>
      </div>
    </div>
  );
};

const ChevronRight = ({ size }) => (
  <svg width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="m9 18 6-6-6-6"/></svg>
);

export default IdentityVault;
