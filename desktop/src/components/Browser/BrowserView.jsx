import React, { useState } from 'react';
import { Globe, ArrowLeft, ArrowRight, RotateCw, ExternalLink, Shield, Lock } from 'lucide-react';
import { motion } from 'framer-motion';

const BrowserView = () => {
  const [url, setUrl] = useState('https://github.com/aether-agent/ada');
  const [isNavigating, setIsNavigating] = useState(false);

  const handleNavigate = () => {
    // In a real scenario, this would trigger a Playwright navigation
    setIsNavigating(true);
    setTimeout(() => setIsNavigating(false), 1500);
  };

  return (
    <div className="flex flex-col h-full bg-aether-space">
      {/* Browser Toolbar */}
      <div className="h-12 border-b border-white/5 flex items-center gap-4 px-4 bg-white/5 backdrop-blur-md">
        <div className="flex items-center gap-1">
          <button className="p-1.5 hover:bg-white/5 rounded-lg text-white/40 hover:text-white transition-colors">
            <ArrowLeft size={16} />
          </button>
          <button className="p-1.5 hover:bg-white/5 rounded-lg text-white/40 hover:text-white transition-colors">
            <ArrowRight size={16} />
          </button>
          <button className="p-1.5 hover:bg-white/5 rounded-lg text-white/40 hover:text-white transition-colors">
            <RotateCw size={16} className={isNavigating ? 'animate-spin' : ''} />
          </button>
        </div>

        <div className="flex-1 max-w-2xl flex items-center bg-black/40 border border-white/10 rounded-full px-3 py-1 gap-2 focus-within:border-aether-indigo/50 transition-colors">
          <Lock size={12} className="text-green-500/60" />
          <input 
            type="text" 
            value={url}
            onChange={(e) => setUrl(e.target.value)}
            onKeyDown={(e) => e.key === 'Enter' && handleNavigate()}
            className="bg-transparent border-none focus:ring-0 text-xs w-full text-white/60 placeholder:text-white/20"
          />
          <Globe size={14} className="text-white/20" />
        </div>

        <div className="flex items-center gap-2 ml-auto">
          <div className="flex items-center gap-1.5 px-2 py-1 rounded-md bg-green-500/10 border border-green-500/20">
             <Shield size={12} className="text-green-500" />
             <span className="text-[10px] font-bold text-green-500 uppercase tracking-wider">Isolated</span>
          </div>
          <button className="p-1.5 hover:bg-white/5 rounded-lg text-white/40 hover:text-white transition-colors">
            <ExternalLink size={16} />
          </button>
        </div>
      </div>

      {/* Browser Content (Placeholder for WebView or Screenshot) */}
      <div className="flex-1 relative overflow-hidden group">
        <div className="absolute inset-0 bg-gradient-to-br from-aether-indigo/5 to-transparent pointer-events-none" />
        
        {/* Placeholder for the actual browser viewport */}
        <div className="w-full h-full flex items-center justify-center p-12">
           <div className="max-w-4xl w-full aspect-video bg-black/40 border border-white/10 rounded-2xl shadow-2xl overflow-hidden relative group">
              <div className="absolute inset-0 flex items-center justify-center opacity-40 group-hover:opacity-100 transition-opacity">
                 <div className="text-center">
                    <div className="w-16 h-16 rounded-full bg-aether-indigo/20 flex items-center justify-center mx-auto mb-4 border border-aether-indigo/30">
                       <Globe size={32} className="text-aether-indigo animate-pulse" />
                    </div>
                    <h3 className="text-lg font-bold text-white/80 mb-2 tracking-tight">CefSharp/WebLayer Viewport</h3>
                    <p className="text-xs text-white/40 max-w-md mx-auto leading-relaxed">
                      The browser instance is running in a low-privilege container. 
                      User interaction events are bridged via the Aether Kernel.
                    </p>
                 </div>
              </div>
              
              {/* Simulated Loading Bar */}
              {isNavigating && (
                <div className="absolute top-0 left-0 right-0 h-0.5 bg-aether-indigo overflow-hidden">
                   <motion.div 
                     initial={{ x: '-100%' }}
                     animate={{ x: '100%' }}
                     transition={{ duration: 1, repeat: Infinity, ease: 'linear' }}
                     className="w-1/3 h-full bg-white/40 shadow-[0_0_15px_rgba(255,255,255,0.5)]"
                   />
                </div>
              )}
           </div>
        </div>

        {/* Footer info */}
        <div className="absolute bottom-4 left-4 right-4 flex justify-between items-center px-4 py-2 bg-black/40 backdrop-blur-xl border border-white/5 rounded-xl">
           <div className="flex items-center gap-3">
              <div className="w-2 h-2 rounded-full bg-green-500" />
              <span className="text-[10px] font-bold text-white/40 uppercase tracking-widest">Runtime: Playwright-Aether</span>
           </div>
           <div className="text-[10px] text-white/20 font-mono">
              PID: 8842 • Mem: 142MB • GPU: Accel
           </div>
        </div>
      </div>
    </div>
  );
};

export default BrowserView;
