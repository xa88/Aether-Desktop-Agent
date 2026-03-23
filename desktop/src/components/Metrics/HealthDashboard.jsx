import React, { useState, useEffect } from 'react';
import { BarChart3, TrendingUp, Cpu, Database, Activity, Globe, MessageSquare } from 'lucide-react';
import { motion } from 'framer-motion';

const HealthDashboard = () => {
  const [metrics, setMetrics] = useState({
    tokens: 42109,
    cost: '$1.24',
    latency: '342ms',
    cacheHit: '0%',
    activeTasks: 0,
    cpu: 0,
    memory: 0,
  });

  const [sandbox, setSandbox] = useState({ provider: 'Loading...', status: 'Initializing' });

  useEffect(() => {
    const updateMetrics = async () => {
      try {
        const data = await window.ada.getHealthMetrics();
        const sb = await window.ada.getSandboxStatus();
        
        setMetrics(prev => ({
          ...prev,
          cacheHit: data.cache_hits + data.cache_misses > 0 
            ? ((data.cache_hits / (data.cache_hits + data.cache_misses)) * 100).toFixed(1) + '%'
            : '0%',
          latency: (data.time_saved_ms / (data.cache_hits || 1)).toFixed(0) + 'ms',
          cpu: data.cpu_usage,
          memory: data.memory_usage,
        }));
        setSandbox(sb);
      } catch (e) {
        console.error("Failed to fetch health metrics", e);
      }
    };

    const interval = setInterval(updateMetrics, 2000);
    updateMetrics(); // Initial call
    return () => clearInterval(interval);
  }, []);

  const stats = [
    { label: 'LLM Token Usage', value: metrics.tokens.toLocaleString(), icon: <MessageChart value={70} />, color: 'text-aether-indigo' },
    { label: 'Cloud API Cost', value: metrics.cost, icon: <TrendingUp size={20} />, color: 'text-green-500' },
    { label: 'Avg Latency (Saved)', value: metrics.latency, icon: <Activity size={20} />, color: 'text-orange-500' },
    { label: 'Vector Cache Hit', value: metrics.cacheHit, icon: <Database size={20} />, color: 'text-blue-500' },
    { label: 'Sandbox Provider', value: sandbox.provider, icon: <Globe size={20} />, color: 'text-purple-400' },
  ];

  return (
    <div className="flex h-full bg-aether-space flex-col overflow-y-auto custom-scrollbar p-8">
      <header className="mb-10">
         <h2 className="text-2xl font-bold mb-2 tracking-tight">System Sovereignty</h2>
         <p className="text-sm text-white/40">Real-time health audit of ADA core runtime and LLM orchestration.</p>
      </header>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-12">
         {stats.map((stat, i) => (
           <motion.div 
             key={stat.label}
             initial={{ opacity: 0, y: 20 }}
             animate={{ opacity: 1, y: 0 }}
             transition={{ delay: i * 0.1 }}
             className="p-6 rounded-3xl bg-white/5 border border-white/5 relative overflow-hidden group hover:border-white/10 transition-colors shadow-2xl"
           >
             <div className="absolute top-0 right-0 p-6 opacity-10 group-hover:opacity-20 transition-opacity">
                {stat.icon}
             </div>
             <div className="relative z-10">
                <div className={`text-[10px] font-bold uppercase tracking-[0.2em] mb-4 ${stat.color}`}>{stat.label}</div>
                <div className="text-3xl font-bold text-white/90 tabular-nums">{stat.value}</div>
             </div>
             <div className="absolute bottom-0 left-0 h-1 bg-gradient-to-r from-aether-indigo to-transparent w-full opacity-0 group-hover:opacity-100 transition-opacity" />
           </motion.div>
         ))}
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
         <div className="lg:col-span-2 space-y-6">
            <div className="p-8 rounded-3xl bg-white/5 border border-white/5">
                <div className="flex items-center justify-between mb-8">
                   <h3 className="text-sm font-bold uppercase tracking-widest text-white/60">Execution Timeline</h3>
                   <div className="flex items-center gap-4 text-[10px] font-bold text-white/20">
                      <span className="flex items-center gap-1.5"><div className="w-1.5 h-1.5 rounded-full bg-aether-indigo" /> Planning</span>
                      <span className="flex items-center gap-1.5"><div className="w-1.5 h-1.5 rounded-full bg-green-500" /> Success</span>
                   </div>
                </div>
                {/* Simulated Chart Placeholder */}
                <div className="h-48 flex items-end justify-between gap-1 px-2">
                   {[40, 60, 45, 80, 55, 90, 70, 40, 85, 50, 65, 95, 30, 75, 55, 80].map((h, i) => (
                      <motion.div 
                        key={i} 
                        initial={{ height: 0 }}
                        animate={{ height: `${h}%` }}
                        transition={{ delay: i * 0.05, duration: 1 }}
                        className={`w-full rounded-t-sm ${i % 3 === 0 ? 'bg-aether-indigo/40' : 'bg-white/10'}`} 
                      />
                   ))}
                </div>
            </div>
         </div>

         <div className="space-y-6">
            <div className="p-6 rounded-3xl bg-black/40 border border-white/5 h-full">
               <h3 className="text-sm font-bold uppercase tracking-widest text-white/60 mb-6">Resource Guard</h3>
               <div className="space-y-6">
                  <ResourceItem label="CPU Usage" value={metrics.cpu} color="bg-aether-indigo" icon={<Cpu size={14} />} />
                  <ResourceItem label="Memory" value={metrics.memory} color="bg-blue-500" icon={<Database size={14} />} />
                  <ResourceItem label="Sandbox Status" value={sandbox.status === 'Ready' ? 100 : 0} color="bg-green-500" icon={<Globe size={14} />} customValue={sandbox.status} />
               </div>
            </div>
         </div>
      </div>
    </div>
  );
};

const MessageChart = ({ value }) => (
  <div className="relative w-12 h-12">
     <svg viewBox="0 0 36 36" className="w-full h-full transform -rotate-90">
        <path d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831" fill="none" stroke="rgba(255,255,255,0.05)" strokeWidth="3" />
        <path d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831" fill="none" stroke="currentColor" strokeWidth="3" strokeDasharray={`${value}, 100`} />
     </svg>
     <div className="absolute inset-0 flex items-center justify-center">
        <MessageSquare size={12} className="opacity-40" />
     </div>
  </div>
);

const ResourceItem = ({ label, value, color, icon }) => (
  <div className="space-y-2">
     <div className="flex justify-between items-center text-[10px] font-bold uppercase tracking-wider text-white/40">
        <div className="flex items-center gap-2">{icon} {label}</div>
        <span>{customValue || `${value}%`}</span>
     </div>
     <div className="h-1.5 w-full bg-white/5 rounded-full overflow-hidden">
        <motion.div 
          initial={{ width: 0 }}
          animate={{ width: `${value}%` }}
          className={`h-full ${color}`} 
        />
     </div>
  </div>
);

export default HealthDashboard;
