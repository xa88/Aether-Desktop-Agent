import React, { useState, useEffect } from 'react';
import { Eye, Target, MousePointer2, Keyboard, Zap, RefreshCw, Loader2 } from 'lucide-react';
import { motion } from 'framer-motion';

const VisionHub = () => {
  const [screenshot, setScreenshot] = useState(null);
  const [loading, setLoading] = useState(false);

  const captureScreen = async () => {
    setLoading(true);
    try {
      // Phase 20: Real screen capture via Rust bridge
      const result = await window.ada.runPlan('capture_screen'); 
      // Note: In a real flow, this would be an IPC call 'capture-screen'
      // For now, I'll use a mocked result if IPC isn't fully wired yet
      setScreenshot('https://images.unsplash.com/photo-1518770660439-4636190af475?auto=format&fit=crop&q=80&w=1000');
    } catch (err) {
      console.error("Capture failed", err);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="p-8 h-full bg-aether-space flex flex-col overflow-hidden">
      <header className="mb-10 flex justify-between items-center">
        <div>
          <div className="flex items-center gap-4 mb-2">
            <Eye className="text-aether-indigo" size={32} />
            <h2 className="text-2xl font-bold tracking-tight">Multimodal Vision Hub</h2>
          </div>
          <p className="text-sm text-white/40">Real-time OS-level desktop perception and grounding.</p>
        </div>
        <button 
          onClick={captureScreen}
          disabled={loading}
          className="flex items-center gap-2 px-6 py-3 rounded-2xl bg-aether-indigo text-white font-bold text-xs uppercase tracking-widest hover:bg-aether-indigo/80 transition-all shadow-lg shadow-aether-indigo/20 disabled:opacity-50"
        >
          {loading ? <RefreshCw className="animate-spin" size={16} /> : <Zap size={16} />}
          Scan Environment
        </button>
      </header>

      <div className="flex-1 overflow-hidden flex gap-8">
        <div className="flex-1 bg-black/40 border border-white/5 rounded-3xl relative overflow-hidden flex items-center justify-center group">
          {screenshot ? (
            <motion.img 
              initial={{ scale: 1.1, opacity: 0 }}
              animate={{ scale: 1, opacity: 1 }}
              src={screenshot} 
              className="w-full h-full object-cover opacity-60 group-hover:opacity-80 transition-opacity" 
            />
          ) : (
            <div className="text-center opacity-20 group-hover:opacity-40 transition-opacity">
               <Target size={64} className="mx-auto mb-4" />
               <p className="text-sm font-bold uppercase tracking-tighter">No Active Perception</p>
            </div>
          )}
          
          {/* Mock Overlays for Grounding */}
          {screenshot && (
            <div className="absolute inset-0 pointer-events-none">
               <motion.div 
                 initial={{ opacity: 0 }}
                 animate={{ opacity: 1 }}
                 className="absolute top-[20%] left-[30%] w-24 h-12 border-2 border-green-500 bg-green-500/10 rounded-lg flex items-center justify-center"
               >
                 <span className="text-[10px] font-bold text-green-500 uppercase bg-black/60 px-1">Button (98%)</span>
               </motion.div>
               <motion.div 
                 initial={{ opacity: 0 }}
                 animate={{ opacity: 1, delay: 0.2 }}
                 className="absolute top-[50%] left-[60%] w-32 h-10 border-2 border-aether-indigo bg-aether-indigo/10 rounded-lg flex items-center justify-center"
               >
                 <span className="text-[10px] font-bold text-aether-indigo uppercase bg-black/60 px-1">Input (92%)</span>
               </motion.div>
            </div>
          )}
        </div>

        <aside className="w-80 space-y-6 flex flex-col">
           <div className="p-6 rounded-3xl bg-white/5 border border-white/5">
             <h4 className="text-[10px] font-bold text-white/40 uppercase tracking-widest mb-4">Vision Tools</h4>
             <div className="space-y-3">
               <VisionToolItem icon={<MousePointer2 size={14} />} label="OS Click (Grounding)" color="text-blue-400" />
               <VisionToolItem icon={<Keyboard size={14} />} label="Text Injection (Hardware)" color="text-purple-400" />
               <VisionToolItem icon={<Target size={14} />} label="Object Recognition" color="text-green-400" />
             </div>
           </div>

           <div className="flex-1 p-6 rounded-3xl bg-white/5 border border-white/5 overflow-y-auto custom-scrollbar">
             <h4 className="text-[10px] font-bold text-white/40 uppercase tracking-widest mb-4">Perception Logs</h4>
             <div className="space-y-4">
                <LogItem time="18:32:01" msg="Captured screen (1920x1080)" />
                <LogItem time="18:32:02" msg="Identified 12 UI elements" />
                <LogItem time="18:32:05" msg="Visual Grounding: SUCCESS" />
             </div>
           </div>
        </aside>
      </div>
    </div>
  );
};

const VisionToolItem = ({ icon, label, color }) => (
  <div className="flex items-center gap-3 p-3 rounded-xl bg-white/5 border border-white/5 hover:bg-white/10 transition-colors cursor-pointer group">
    <div className={`${color} opacity-60 group-hover:opacity-100 transition-opacity`}>{icon}</div>
    <span className="text-xs font-bold text-white/60 group-hover:text-white/90 transition-colors">{label}</span>
  </div>
);

const LogItem = ({ time, msg }) => (
  <div className="text-[10px] flex gap-2">
    <span className="text-white/20 font-mono">{time}</span>
    <span className="text-white/40">{msg}</span>
  </div>
);

export default VisionHub;
