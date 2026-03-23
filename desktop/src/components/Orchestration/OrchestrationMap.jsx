import React, { useState, useEffect } from 'react';
import { Share2, Server, Activity, RotateCcw, ShieldAlert, CheckCircle2, Play, Cpu, Zap } from 'lucide-react';
import { motion, AnimatePresence } from 'framer-motion';

const OrchestrationMap = () => {
  const [swarm, setSwarm] = useState(null);
  const [nodes, setNodes] = useState([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchData = async () => {
      try {
        if (window.ada) {
          const [swarmData, nodeData] = await Promise.all([
            window.ada.getSwarmStatus(),
            window.ada.getClusterNodes()
          ]);
          setSwarm(swarmData);
          setNodes(nodeData);
        }
      } catch (err) {
        console.error("Failed to fetch swarm/cluster data", err);
      } finally {
        setLoading(false);
      }
    };

    fetchData(); // Initial fetch
    const interval = setInterval(fetchData, 3000); // Periodic fetch
    return () => clearInterval(interval); // Cleanup
  }, []);

  if (!swarm) return <div className="p-8 text-white/20">Loading Swarm Matrix...</div>;

  return (
    <div className="flex h-full bg-aether-space flex-col p-8 overflow-y-auto custom-scrollbar">
      <header className="mb-10 flex justify-between items-end">
        <div>
          <h2 className="text-2xl font-bold mb-2 tracking-tight">Swarm Orchestration</h2>
          <p className="text-sm text-white/40">Real-time visualization of distributed agent workers and task synthesis.</p>
        </div>
        <div className="flex gap-4">
          <div className="px-4 py-2 rounded-xl bg-white/5 border border-white/5 flex items-center gap-3">
            <Share2 size={16} className="text-aether-indigo" />
            <span className="text-xs font-bold">{swarm.active_agents} Active Workers</span>
          </div>
        </div>
      </header>

      <div className="flex gap-8 flex-1 min-h-0">
        <div className="flex-1 flex flex-col min-w-0">
          {/* Task Flow Visualization & Logic Graph */}
          <div className="flex-1 min-h-[400px] mb-8 relative p-12 border border-white/5 rounded-[40px] bg-black/40 overflow-hidden">
            {/* Background Grid */}
            <div className="absolute inset-0 opacity-10 pointer-events-none" style={{ backgroundImage: 'radial-gradient(circle, #4f46e5 1px, transparent 1px)', backgroundSize: '40px 40px' }} />
            
            <svg className="absolute inset-0 w-full h-full pointer-events-none">
              <motion.path 
                d="M 50% 100 L 20% 300 M 50% 100 L 50% 300 M 50% 100 L 80% 300" 
                stroke="url(#gradient)" strokeWidth="1" fill="none" opacity="0.2"
                initial={{ pathLength: 0 }} animate={{ pathLength: 1 }}
              />
              <defs>
                <linearGradient id="gradient" x1="0%" y1="0%" x2="0%" y2="100%">
                  <stop offset="0%" stopColor="#4f46e5" />
                  <stop offset="100%" stopColor="#818cf8" />
                </linearGradient>
              </defs>
            </svg>

            {/* Nodes */}
            <div className="absolute left-1/2 -translate-x-1/2 top-10 z-20">
              <AgentNode agent={swarm.workers.find(w => w.id === 'director')} isDirector />
            </div>

            <div className="absolute inset-x-0 bottom-20 flex justify-around px-8">
               {swarm.workers.filter(w => w.id !== 'director').map((w, i) => (
                 <AgentNode key={w.id} agent={w} delay={i * 0.1} />
               ))}
            </div>
          </div>

          {/* Detailed Status List */}
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {swarm.workers.map(w => (
              <WorkerDetailCard key={w.id} agent={w} />
            ))}
          </div>
        </div>

        {/* Right Sidebar: Cluster Nodes */}
        <aside className="w-80 flex flex-col gap-6">
          <div className="p-6 rounded-[32px] bg-white/5 border border-white/5 backdrop-blur-md">
            <div className="flex items-center gap-3 mb-6">
              <div className="w-8 h-8 rounded-xl bg-aether-indigo/20 flex items-center justify-center">
                <Zap size={16} className="text-aether-indigo" />
              </div>
              <h3 className="text-sm font-bold tracking-tight">Swarm Cluster</h3>
            </div>
            <div className="space-y-4">
              {nodes.map(node => (
                <div key={node.id} className="p-4 rounded-2xl bg-black/40 border border-white/5 hover:border-white/10 transition-all group">
                  <div className="flex items-center justify-between mb-2">
                    <span className="text-[10px] font-bold text-white/80 group-hover:text-aether-indigo transition-colors">{node.hostname}</span>
                    <div className={`w-1.5 h-1.5 rounded-full ${node.status === 'Online' ? 'bg-green-500 shadow-[0_0_8px_rgba(34,197,94,0.4)]' : 'bg-orange-500'}`} />
                  </div>
                  <div className="flex justify-between items-center text-[9px] text-white/30">
                    <span className="font-mono">{node.ip}</span>
                    <span className="uppercase tracking-widest">{node.status}</span>
                  </div>
                </div>
              ))}
            </div>
            <button className="w-full mt-6 py-3 rounded-xl bg-white/5 border border-dashed border-white/10 text-[10px] font-bold text-white/20 hover:text-white/40 hover:border-white/20 transition-all uppercase tracking-widest">
              + Connect Remote Node
            </button>
          </div>

          <div className="p-6 rounded-[32px] bg-aether-indigo/10 border border-aether-indigo/20">
            <h4 className="text-[10px] font-bold text-aether-indigo uppercase tracking-widest mb-2">Cluster Load</h4>
            <div className="text-2xl font-bold text-white mb-4">42.8%</div>
            <div className="h-1.5 w-full bg-white/5 rounded-full overflow-hidden">
               <motion.div initial={{ width: 0 }} animate={{ width: '42.8%' }} className="h-full bg-aether-indigo shadow-[0_0_10px_rgba(79,70,229,0.5)]" />
            </div>
          </div>
        </aside>
      </div>
    </div>
  );
};

const AgentNode = ({ agent, isDirector, delay = 0 }) => {
  if (!agent) return null;
  const statusColors = {
    'Success': 'border-green-500 shadow-green-500/20',
    'Executing': 'border-aether-indigo shadow-aether-indigo/20 animate-pulse',
    'Planning': 'border-blue-500 shadow-blue-500/20',
    'Failed': 'border-red-500 shadow-red-500/20',
    'Idle': 'border-white/10'
  };

  return (
    <motion.div 
      initial={{ opacity: 0, scale: 0.8 }}
      animate={{ opacity: 1, scale: 1 }}
      transition={{ delay }}
      className={`relative p-1 rounded-full border-2 bg-black shadow-xl ${statusColors[agent.status] || ''}`}
    >
      <div className={`w-14 h-14 rounded-full flex items-center justify-center bg-gradient-to-br ${isDirector ? 'from-aether-indigo to-aether-electric' : 'from-white/5 to-white/10'}`}>
        {isDirector ? <Server size={24} className="text-white" /> : <Activity size={20} className="text-white/60" />}
      </div>
      <div className="absolute -bottom-10 left-1/2 -translate-x-1/2 whitespace-nowrap text-center">
        <div className="text-[10px] font-bold text-white/90">{agent.name}</div>
        <div className="text-[8px] text-white/30 uppercase tracking-widest">{agent.status}</div>
      </div>
    </motion.div>
  );
}

const WorkerDetailCard = ({ agent }) => {
  const handleRestart = () => window.ada.restartAgent(agent.id);

  return (
    <div className="p-6 rounded-3xl bg-white/5 border border-white/5 hover:border-white/10 transition-colors">
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-3">
          <div className="w-8 h-8 rounded-lg bg-white/5 flex items-center justify-center">
            {agent.status === 'Success' ? <CheckCircle2 size={16} className="text-green-500" /> : <Play size={16} className="text-aether-indigo" />}
          </div>
          <div>
            <h4 className="text-xs font-bold text-white/80">{agent.name}</h4>
            <span className="text-[8px] text-white/20 font-mono tracking-tighter uppercase">{agent.id}</span>
          </div>
        </div>
        {agent.status === 'Failed' && (
          <button onClick={handleRestart} className="p-2 rounded-lg bg-red-500/10 text-red-500 hover:bg-red-500/20 transition-colors">
            <RotateCcw size={14} />
          </button>
        )}
      </div>

      <div className="space-y-3">
        <div className="flex justify-between text-[10px]">
          <span className="text-white/40">Current Task</span>
          <span className="text-white/80 truncate max-w-[150px]">{agent.current_task || 'Idle'}</span>
        </div>
        <div className="h-1 w-full bg-white/5 rounded-full overflow-hidden">
          <motion.div 
            initial={{ width: 0 }}
            animate={{ width: `${agent.progress * 100}%` }}
            className={`h-full ${agent.status === 'Failed' ? 'bg-red-500' : 'bg-aether-indigo'}`}
          />
        </div>
      </div>
    </div>
  );
};

export default OrchestrationMap;
