import React, { useState, useEffect } from 'react';
import { Clock, CheckCircle, XCircle, ChevronRight, Tool, Play } from 'lucide-react';
import { motion } from 'framer-motion';

const AuditReplay = () => {
  const [auditLogs, setAuditLogs] = useState([]);
  const [selectedRun, setSelectedRun] = useState(null);

  useEffect(() => {
    const fetchLogs = async () => {
      if (window.ada && window.ada.readAuditLog) {
        const logs = await window.ada.readAuditLog();
        setAuditLogs(logs);
        
        // Group by run_id
        const grouped = logs.reduce((acc, log) => {
          if (!acc[log.run_id]) acc[log.run_id] = [];
          acc[log.run_id].push(log);
          return acc;
        }, {});
        
        const runs = Object.keys(grouped).map(id => ({
          id,
          ts: grouped[id][0].ts,
          steps: grouped[id]
        })).sort((a, b) => new Date(b.ts) - new Date(a.ts));
        
        setSelectedRun(runs[0]);
      }
    };
    fetchLogs();

    // Phase 6: Real-time Event Streaming
    if (window.ada && window.ada.onAuditEvent) {
      window.ada.onAuditEvent((event) => {
        setAuditLogs(prev => [...prev, event]);
        if (selectedRun && event.run_id === selectedRun.id) {
           setSelectedRun(prev => ({
             ...prev,
             steps: [...prev.steps, event]
           }));
        }
      });
    }
  }, [selectedRun]);

  if (!selectedRun) {
    return <div className="text-white/20 italic p-8 text-center">No historical audit logs found. Execute a plan to generate data.</div>;
  }

  return (
    <div className="flex h-full overflow-hidden">
      {/* Run Selection Sidebar */}
      <div className="w-64 border-r border-white/5 bg-black/20 p-4 overflow-y-auto custom-scrollbar">
         <div className="text-[10px] font-bold text-white/30 uppercase tracking-[0.2em] mb-4">Past Sessions</div>
         <div className="space-y-2">
            {/* We could implement easier run selection here if we have multiple runs */}
            <div className="p-3 rounded-xl bg-aether-indigo/10 border border-aether-indigo/20">
               <div className="flex items-center gap-2 mb-1">
                 <Play size={12} className="text-aether-indigo shadow-glow" />
                 <span className="text-xs font-bold text-white/80">Active Session</span>
               </div>
               <div className="text-[10px] text-white/40 truncate">{selectedRun.id}</div>
            </div>
         </div>
      </div>

      {/* Replay Timeline */}
      <div className="flex-1 overflow-y-auto p-8 custom-scrollbar">
         <div className="flex items-center justify-between mb-8">
            <h2 className="text-xl font-bold">Execution Timeline</h2>
            <div className="flex items-center gap-2 px-3 py-1 rounded-full bg-white/5 border border-white/5">
                <Clock size={12} className="text-white/40" />
                <span className="text-[10px] text-white/60 font-medium">{new Date(selectedRun.ts).toLocaleString()}</span>
            </div>
         </div>

         <div className="space-y-6 relative">
            <div className="absolute left-[20px] top-4 bottom-4 w-px bg-gradient-to-b from-aether-indigo/40 via-white/5 to-transparent" />
            
            {selectedRun.steps.map((step, i) => (
              <motion.div 
                key={step.step_id}
                initial={{ opacity: 0, x: -10 }}
                animate={{ opacity: 1, x: 0 }}
                transition={{ delay: i * 0.1 }}
                className="relative pl-12 flex flex-col gap-2"
              >
                <div className={`absolute left-0 top-0 w-10 h-10 rounded-xl flex items-center justify-center z-10 shadow-lg ${
                  step.result.status === 'success' ? 'bg-green-500/20 text-green-500 shadow-green-500/10' : 'bg-red-500/20 text-red-500 shadow-red-500/10'
                }`}>
                  {step.result.status === 'success' ? <CheckCircle size={20} /> : <XCircle size={20} />}
                </div>

                <div className="p-4 rounded-2xl bg-white/5 border border-white/10 hover:border-aether-indigo/30 transition-colors group">
                  <div className="flex items-center justify-between mb-2">
                    <div className="flex items-center gap-3">
                      <span className="text-[10px] font-bold text-aether-indigo uppercase tracking-wider">{step.step_id}</span>
                      <h4 className="text-sm font-semibold">{step.tool}</h4>
                    </div>
                    <span className="text-[10px] text-white/30 font-mono">{step.duration_ms}ms</span>
                  </div>
                  
                  <div className="flex items-center gap-2 mb-3">
                    <div className="px-1.5 py-0.5 rounded bg-white/5 text-[10px] text-white/40 font-mono">{step.action}</div>
                    <div className="px-1.5 py-0.5 rounded bg-white/5 text-[10px] text-white/40 font-mono italic">Tier: {step.risk_tier}</div>
                  </div>

                  {step.result.reason && (
                    <div className="mt-3 p-3 rounded-lg bg-red-500/5 border border-red-500/10 text-[11px] text-red-400 font-mono whitespace-pre-wrap">
                       {step.result.reason}
                    </div>
                  )}
                  
                  {step.result.status === 'success' && (
                    <div className="text-[10px] text-green-500/60 font-medium flex items-center gap-1.5 mt-2">
                       <CheckCircle size={12} />
                       Validation Passed
                    </div>
                  )}
                </div>
              </motion.div>
            ))}
         </div>
      </div>
    </div>
  );
};

export default AuditReplay;
