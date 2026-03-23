import React, { useState } from 'react';
import { Play, Plus, Trash2, ChevronRight, Zap, Settings, Save, MoveHorizontal } from 'lucide-react';
import { motion } from 'framer-motion';

const WorkflowStudio = () => {
  const [steps, setSteps] = useState([
    { id: 1, type: 'trigger', label: 'On File Created', config: 'path: /src/*.rs', icon: <Zap size={14} className="text-yellow-500" /> },
    { id: 2, type: 'action', label: 'Analyze with GPT-4', config: 'prompt: "Review logic"', icon: <Settings size={14} className="text-aether-indigo" /> },
    { id: 3, type: 'action', label: 'Run Rust Tests', config: 'args: --nocapture', icon: <Play size={14} className="text-green-500" /> },
  ]);

  const addStep = () => {
    setSteps([...steps, { id: Date.now(), type: 'action', label: 'New Action', config: 'Edit config...', icon: <Plus size={14} /> }]);
  };

  return (
    <div className="flex h-full bg-aether-space flex-col">
      <header className="h-16 border-b border-white/5 flex items-center justify-between px-8 bg-black/20">
         <div className="flex items-center gap-4">
            <h2 className="text-xl font-bold">Workflow Studio</h2>
            <div className="px-2 py-0.5 rounded bg-white/5 border border-white/10 text-[10px] text-white/40 uppercase font-bold tracking-widest">Active</div>
         </div>
         <div className="flex items-center gap-3">
            <button className="flex items-center gap-2 px-4 py-2 hover:bg-white/5 rounded-xl text-xs font-bold transition-all text-white/60">
               <Save size={16} />
               Save Draft
            </button>
            <button className="flex items-center gap-2 px-4 py-2 bg-aether-indigo hover:bg-aether-indigo/80 rounded-xl text-xs font-bold transition-all shadow-lg shadow-aether-indigo/20">
               <Play size={16} className="fill-current" />
               Deploy Playbook
            </button>
         </div>
      </header>

      <div className="flex-1 overflow-hidden flex">
         {/* Step Library */}
         <div className="w-72 border-r border-white/5 bg-black/20 p-6 flex flex-col gap-6">
            <div className="space-y-4">
               <h4 className="text-[10px] font-bold text-white/30 uppercase tracking-[0.2em]">Triggers</h4>
               {['File Event', 'Cron Schedule', 'HTTP Hook', 'Manual'].map(t => (
                  <div key={t} className="p-3 rounded-xl bg-white/5 border border-white/5 hover:border-aether-indigo/30 cursor-pointer transition-all flex items-center gap-3 group">
                     <div className="w-8 h-8 rounded-lg bg-black/40 flex items-center justify-center text-white/20 group-hover:text-aether-indigo transition-colors"><Zap size={16} /></div>
                     <span className="text-xs font-medium text-white/60">{t}</span>
                  </div>
               ))}
            </div>

            <div className="space-y-4">
               <h4 className="text-[10px] font-bold text-white/30 uppercase tracking-[0.2em]">Core Actions</h4>
               {['Code Execution', 'LLM Query', 'FS Move', 'Notification'].map(a => (
                  <div key={a} className="p-3 rounded-xl bg-white/5 border border-white/5 hover:border-aether-indigo/30 cursor-pointer transition-all flex items-center gap-3 group">
                     <div className="w-8 h-8 rounded-lg bg-black/40 flex items-center justify-center text-white/20 group-hover:text-aether-indigo transition-colors"><MoveHorizontal size={16} /></div>
                     <span className="text-xs font-medium text-white/60">{a}</span>
                  </div>
               ))}
            </div>
         </div>

         {/* Canvas Area */}
         <div className="flex-1 bg-black/40 p-12 overflow-y-auto relative custom-scrollbar">
            <div className="max-w-xl mx-auto space-y-4 relative">
               {/* Vertical Connection Line */}
               <div className="absolute left-[23px] top-6 bottom-6 w-0.5 bg-gradient-to-b from-aether-indigo/50 to-transparent pointer-events-none" />

               {steps.map((step, i) => (
                  <motion.div 
                    key={step.id}
                    initial={{ opacity: 0, x: -20 }}
                    animate={{ opacity: 1, x: 0 }}
                    transition={{ delay: i * 0.1 }}
                    className="flex gap-6 group relative"
                  >
                     <div className="w-12 h-12 rounded-2xl bg-white/5 border border-white/10 flex items-center justify-center relative z-10 group-hover:border-aether-indigo/50 transition-colors bg-aether-space shadow-xl">
                        {step.icon}
                     </div>
                     <div className="flex-1 p-5 bg-white/5 border border-white/5 rounded-2xl hover:border-white/10 transition-all">
                        <div className="flex items-center justify-between mb-2">
                           <h3 className="text-sm font-bold text-white/90">{step.label}</h3>
                           <button className="text-white/10 hover:text-red-500/60 transition-colors">
                              <Trash2 size={14} />
                           </button>
                        </div>
                        <p className="text-[11px] font-mono text-white/30">{step.config}</p>
                     </div>
                  </motion.div>
               ))}

               <button 
                 onClick={addStep}
                 className="flex items-center gap-3 px-6 py-4 rounded-2xl border border-dashed border-white/10 text-white/20 hover:text-white/60 hover:border-white/30 transition-all w-full justify-center group"
               >
                  <Plus size={18} className="group-hover:scale-110 transition-transform" />
                  <span className="text-xs font-bold uppercase tracking-widest">Add Automation Step</span>
               </button>
            </div>
         </div>
      </div>
    </div>
  );
};

export default WorkflowStudio;
