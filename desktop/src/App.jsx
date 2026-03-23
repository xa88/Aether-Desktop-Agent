import React, { useState, useEffect } from 'react';
import { 
  MessageSquare, 
  Mic, 
  Settings, 
  Code, 
  Terminal, 
  Zap, 
  ChevronRight, 
  Search,
  Cpu,
  Shield,
  Activity,
  User,
  Bot,
  Globe,
  Image as ImageIcon,
  Play,
  Puzzle,
  TrendingUp,
  Box,
  Lock
} from 'lucide-react';
import { motion, AnimatePresence } from 'framer-motion';
import FileTree from './components/IDE/FileTree';
import MonacoEditor from './components/IDE/MonacoEditor';
import AuditReplay from './components/Audit/AuditReplay';
import BrowserView from './components/Browser/BrowserView';
import VisionGallery from './components/Vision/VisionGallery';
import TerminalView from './components/Terminal/TerminalView';
import PluginManager from './components/Plugins/PluginManager';
import WorkflowStudio from './components/Workflows/WorkflowStudio';
import HealthDashboard from './components/Metrics/HealthDashboard';
import CommandPalette from './components/Navigation/CommandPalette';
import SettingsPanel from './components/Settings/SettingsPanel';
import ArtifactExplorer from './components/Artifacts/ArtifactExplorer';
import IdentityVault from './components/Security/IdentityVault';
import OrchestrationMap from './components/Orchestration/OrchestrationMap';
import MemoryExplorer from './components/Memory/MemoryExplorer';
import PluginExplorer from './components/Plugins/PluginExplorer';
import VisionHub from './components/Vision/VisionHub';

const App = () => {
  const [activeTab, setActiveTab] = useState('chat');
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);
  const [messages, setMessages] = useState([
    { id: 1, role: 'bot', content: 'Hello! I am ADA, your Aether Desktop Agent. How can I assist you today?' }
  ]);
  const [inputValue, setInputValue] = useState('');
  const [isRecording, setIsRecording] = useState(false);
  const [currentFile, setCurrentFile] = useState(null);
  const [fileContent, setFileContent] = useState('');
  const [isModified, setIsModified] = useState(false);
  const [isCommandPaletteOpen, setIsCommandPaletteOpen] = useState(false);

  const tabs = [
    { id: 'chat', icon: <MessageSquare size={20} />, label: 'Chat' },
    { id: 'ide', icon: <Code size={20} />, label: 'IDE' },
    { id: 'workflows', icon: <Zap size={20} />, label: 'Playbooks' },
    { id: 'browser', icon: <Globe size={20} />, label: 'Browser' },
    { id: 'vision', icon: <ImageIcon size={20} />, label: 'Vision' },
    { id: 'health', icon: <TrendingUp size={20} />, label: 'Health' },
    { id: 'artifacts', icon: <Box size={20} />, label: 'Artifacts' },
    { id: 'vault', icon: <Lock size={20} />, label: 'Vault' },
    { id: 'plugins', icon: <Puzzle size={20} />, label: 'Plugins' },
    { id: 'voice', icon: <Mic size={20} />, label: 'Voice' },
    { id: 'orchestration', icon: <Share2 size={20} />, label: 'Swarm' },
    { id: 'memory', icon: <Activity size={20} />, label: 'Memory' },
    { id: 'audit', icon: <Activity size={20} />, label: 'Audit' },
  ];

  const [swarmStats, setSwarmStats] = useState({ active_agents: 0 });

  useEffect(() => {
    const fetchSwarm = async () => {
      try {
        const stats = await window.ada.getSwarmStatus();
        setSwarmStats(stats);
      } catch (e) {}
    };
    const inv = setInterval(fetchSwarm, 5000);
    fetchSwarm();
    return () => clearInterval(inv);
  }, []);

  const [logs, setLogs] = useState([]);

  useEffect(() => {
    // Listen for real-time stdout from ADA CLI
    if (window.ada && window.ada.onPlanStdout) {
      window.ada.onPlanStdout((_event, text) => {
        setLogs(prev => [...prev.slice(-100), { time: new Date().toLocaleTimeString(), text }]);
      });
    }

    if (window.ada && window.ada.onAuditEvent) {
      window.ada.onAuditEvent((data) => {
        // Map audit events to chat 'thoughts'
        setMessages(prev => [...prev, {
          id: Date.now() + Math.random(),
          role: 'thought',
          content: data.message || data.event_type || "Processing step...",
          timestamp: new Date().toLocaleTimeString()
        }]);
      });
    }
  }, []);

  const handleSend = async () => {
    if (!inputValue.trim()) return;
    const newMsg = { id: Date.now(), role: 'user', content: inputValue };
    setMessages(prev => [...prev, newMsg]);
    setInputValue('');
    
    try {
      // In a real scenario, we might write the message to a file and call runPlan
      // Or we might have a specific 'chat' IPC handler. 
      // For now, let's simulate the ADA core processing it.
      if (window.ada && window.ada.runPlan) {
        // Mock: write user intent to a temporary plan and execute it
        // await window.ada.writeFile('tasks/current_plan.json', JSON.stringify({ goal: inputValue }));
        // await window.ada.runPlan('tasks/current_plan.json');
      }

      setTimeout(() => {
        setMessages(prev => [...prev, { 
          id: Date.now() + 1, 
          role: 'bot', 
          content: `Analysis complete. I've initiated the planning hierarchy for: "${inputValue}". Monitoring filesystem events now.` 
        }]);
      }, 800);
    } catch (err) {
      setMessages(prev => [...prev, { 
        id: Date.now() + 1, 
        role: 'bot', 
        content: `Error communicating with core: ${err.message}` 
      }]);
    }
  };

  const handleSaveSettings = async (config) => {
    try {
      if (window.ada && window.ada.saveLlmConfig) {
        await window.ada.saveLlmConfig(config);
        setIsSettingsOpen(false);
      }
    } catch (err) {
      alert("Failed to save config: " + err.message);
    }
  };

  const handleFileSelect = async (path) => {
    try {
      if (window.ada && window.ada.readFile) {
        const content = await window.ada.readFile(path);
        setCurrentFile(path);
        setFileContent(content);
        setIsModified(false);
        setActiveTab('ide');
      }
    } catch (err) {
      alert("Error opening file: " + err.message);
    }
  };

  const handleFileSave = async () => {
    if (!currentFile) return;
    try {
      if (window.ada && window.ada.writeFile) {
        await window.ada.writeFile(currentFile, fileContent);
        setIsModified(false);
        // Toast notifications would be nice here
      }
    } catch (err) {
      alert("Error saving file: " + err.message);
    }
  };

  useEffect(() => {
    const handleKeyDown = (e) => {
      if (e.ctrlKey && e.key === 's') {
        e.preventDefault();
        handleFileSave();
      }
      if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
        e.preventDefault();
        setIsCommandPaletteOpen(true);
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [currentFile, fileContent]);

  const handleRunCode = async () => {
    if (!currentFile) return;
    try {
      if (window.ada && window.ada.runInSandbox) {
        await window.ada.runInSandbox(fileContent, currentFile.split('.').pop());
      }
    } catch (err) {
      alert("Execution failed: " + err.message);
    }
  };

  const handleVoiceCapture = async () => {
    if (isRecording) return;
    setIsRecording(true);
    try {
      if (window.ada && window.ada.captureVoice) {
        const text = await window.ada.captureVoice();
        if (text) {
          setInputValue(prev => prev + (prev ? ' ' : '') + text);
        }
      }
    } catch (err) {
      alert("Voice capture failed: " + err.message);
    } finally {
      setIsRecording(false);
    }
  };

  return (
    <div className="flex h-screen bg-aether-space text-aether-ghost overflow-hidden font-sans">
      {/* Sidebar */}
      <aside className="w-20 lg:w-64 border-right border-aether-border bg-aether-surface/50 backdrop-blur-xl flex flex-col items-center py-6">
        <div className="mb-10 flex items-center gap-3 px-6 w-full">
          <div className="w-10 h-10 rounded-xl bg-gradient-to-br from-aether-indigo to-aether-electric flex items-center justify-center shadow-lg shadow-aether-indigo/20">
            <Zap className="text-white fill-white" size={24} />
          </div>
          <h1 className="text-xl font-bold tracking-tight hidden lg:block bg-clip-text text-transparent bg-gradient-to-r from-white to-white/60">ADA</h1>
        </div>

        <nav className="flex-1 w-full px-3 space-y-2">
          {tabs.map(tab => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id)}
              className={`w-full flex items-center gap-3 p-3 rounded-xl transition-all duration-200 group ${
                activeTab === tab.id 
                ? 'bg-aether-indigo/10 text-aether-indigo' 
                : 'hover:bg-white/5 text-aether-ghost/60 hover:text-white'
              }`}
            >
              <span className={`${activeTab === tab.id ? 'scale-110' : ''} transition-transform`}>
                {tab.icon}
              </span>
              <span className="font-medium hidden lg:block">{tab.label}</span>
              {activeTab === tab.id && (
                <motion.div 
                  layoutId="activeTab"
                  className="ml-auto w-1.5 h-1.5 rounded-full bg-aether-indigo hidden lg:block"
                />
              )}
            </button>
          ))}
        </nav>

        <div className="mt-auto w-full px-3 space-y-2">
          <button 
            onClick={() => setIsSettingsOpen(true)}
            className="w-full flex items-center gap-3 p-3 rounded-xl hover:bg-white/5 text-aether-ghost/60 hover:text-white transition-all group"
          >
            <Settings size={20} className="group-hover:rotate-45 transition-transform duration-500" />
            <span className="font-medium hidden lg:block">Settings</span>
          </button>
          <div className="p-4 rounded-2xl bg-gradient-to-br from-white/5 to-transparent border border-white/5 hidden lg:block">
            <div className="flex items-center justify-between mb-2">
              <span className="text-[10px] font-semibold text-white/40 uppercase tracking-widest">Context Usage</span>
              <span className="text-[10px] text-aether-indigo font-bold">4.2x</span>
            </div>
            <div className="h-1.5 w-full bg-white/5 rounded-full overflow-hidden mb-2">
              <motion.div 
                initial={{ width: 0 }}
                animate={{ width: '15%' }}
                className="h-full bg-gradient-to-r from-aether-indigo to-aether-electric"
              />
            </div>
            <p className="text-[8px] text-white/20 leading-tight">2.4k / 128k Tokens • Compressed</p>
          </div>

          <div className="p-4 rounded-2xl bg-gradient-to-br from-white/5 to-transparent border border-white/5 hidden lg:block">
            <div className="flex items-center justify-between mb-2">
              <span className="text-[10px] font-semibold text-white/40 uppercase tracking-widest">Swarm Intel</span>
              <span className={`text-[10px] font-bold ${swarmStats.active_agents > 0 ? 'text-aether-indigo' : 'text-white/20'}`}>
                {swarmStats.active_agents} Agents
              </span>
            </div>
            <div className="flex gap-1 mb-2">
              {[...Array(5)].map((_, i) => (
                <div key={i} className={`h-1 flex-1 rounded-full ${i < swarmStats.active_agents ? 'bg-aether-indigo' : 'bg-white/5'}`} />
              ))}
            </div>
            <p className="text-[8px] text-white/20 leading-tight">Master-Slave Protocol • Active</p>
          </div>

          <div className="p-4 rounded-2xl bg-gradient-to-br from-white/5 to-transparent border border-white/5 hidden lg:block">
            <div className="flex items-center gap-2 mb-2">
              <div className="w-2 h-2 rounded-full bg-green-500 animate-pulse" />
              <span className="text-xs font-semibold text-white/40 uppercase tracking-wider">System Ready</span>
            </div>
            <p className="text-[10px] text-white/30 leading-tight">Version 1.0.6-alpha • Swarm Enabled</p>
          </div>
        </div>
      </aside>

      {/* Main Content */}
      <main className="flex-1 flex flex-col relative bg-aether-space content-area">
        {/* Header */}
        <header className="h-16 border-b border-aether-border flex items-center justify-between px-8 bg-aether-space/80 backdrop-blur-md sticky top-0 z-10">
          <div className="flex items-center gap-4">
            <div className="flex items-center gap-2 px-3 py-1.5 rounded-full bg-white/5 border border-white/5">
              <Cpu size={14} className="text-aether-indigo" />
              <span className="text-xs font-medium text-white/70">Model: GPT-4o</span>
              <ChevronRight size={14} className="text-white/20" />
            </div>
          </div>
          <div className="flex items-center gap-3">
             <div className="flex -space-x-2">
               <div className="w-8 h-8 rounded-full border-2 border-aether-space bg-aether-indigo flex items-center justify-center">
                 <Bot size={14} />
               </div>
               <div className="w-8 h-8 rounded-full border-2 border-aether-space bg-zinc-800 flex items-center justify-center">
                 <User size={14} />
               </div>
             </div>
          </div>
        </header>

        {/* Content Tabs */}
        <div className="flex-1 overflow-hidden">
          <AnimatePresence mode="wait">
            {activeTab === 'chat' && (
              <motion.div 
                key="chat"
                initial={{ opacity: 0, y: 10 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, y: -10 }}
                className="h-full flex flex-col max-w-4xl mx-auto w-full"
              >
                <div className="flex-1 overflow-y-auto p-8 space-y-6">
                  {messages.map(msg => (
                    <div key={msg.id} className={`flex ${msg.role === 'user' ? 'justify-end' : 'justify-start'}`}>
                      <div className={`flex gap-4 max-w-[80%] ${msg.role === 'user' ? 'flex-row-reverse' : ''} ${msg.role === 'thought' ? 'w-full opacity-60' : ''}`}>
                        <div className={`w-8 h-8 rounded-lg flex-shrink-0 flex items-center justify-center ${
                          msg.role === 'user' ? 'bg-zinc-800' : 
                          msg.role === 'thought' ? 'bg-white/5 border border-white/10' :
                          'bg-aether-indigo/20 text-aether-indigo border border-aether-indigo/30'
                        }`}>
                          {msg.role === 'user' ? <User size={16} /> : 
                           msg.role === 'thought' ? <Activity size={14} className="text-white/40" /> :
                           <Bot size={16} />}
                        </div>
                        <div className={`p-4 rounded-2xl ${
                          msg.role === 'user' 
                          ? 'bg-aether-indigo text-white rounded-tr-none shadow-lg shadow-aether-indigo/20' 
                          : msg.role === 'thought'
                          ? 'bg-transparent border border-white/5 border-dashed rounded-tl-none italic text-xs'
                          : 'bg-white/5 border border-white/5 rounded-tl-none'
                        }`}>
                          {msg.role === 'thought' && <div className="text-[8px] font-bold uppercase tracking-widest text-white/20 mb-1">ADA Thought Process</div>}
                          <p className={`leading-relaxed ${msg.role === 'thought' ? 'text-white/40' : 'text-sm'}`}>{msg.content}</p>
                        </div>
                      </div>
                    </div>
                  ))}
                </div>

                <div className="p-8 mt-auto">
                  <div className="relative group">
                    <div className="absolute -inset-0.5 bg-gradient-to-r from-aether-indigo to-aether-electric rounded-2xl blur opacity-20 group-focus-within:opacity-40 transition duration-1000"></div>
                    <div className="relative flex items-center bg-aether-surface border border-white/10 rounded-2xl p-2 gap-2 shadow-2xl">
                      <input 
                        type="text"
                        value={inputValue}
                        onChange={(e) => setInputValue(e.target.value)}
                        onKeyDown={(e) => e.key === 'Enter' && handleSend()}
                        placeholder="Message ADA..."
                        className="flex-1 bg-transparent border-none focus:ring-0 text-sm px-4 py-2 placeholder:text-white/20"
                      />
                      <button 
                        onClick={handleVoiceCapture}
                        className={`p-2 rounded-xl transition-all duration-300 ${isRecording ? 'bg-red-500/20 text-red-500 animate-pulse' : 'hover:bg-white/5 text-white/40 hover:text-white'}`}
                      >
                        <Mic size={20} />
                      </button>
                      <button 
                        onClick={handleSend}
                        className="p-2 bg-aether-indigo hover:bg-aether-indigo/80 rounded-xl transition-all shadow-lg shadow-aether-indigo/30"
                      >
                        <Zap size={20} className="fill-white text-white" />
                      </button>
                    </div>
                  </div>
                  <p className="text-[10px] text-center mt-4 text-white/20 font-medium tracking-widest uppercase">
                    Autonomous Desktop Agent • Powering Productivity
                  </p>
                </div>
              </motion.div>
            )}

            {activeTab === 'ide' && (
              <motion.div 
                key="ide"
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
                className="h-full flex"
              >
                <div className="w-64 border-r border-aether-border bg-black/20 flex-shrink-0">
                  <FileTree onFileSelect={handleFileSelect} />
                </div>
                <div className="flex-1 flex flex-col bg-zinc-950/50">
                  {currentFile ? (
                    <>
                      <div className="h-10 px-4 border-b border-white/5 flex items-center justify-between bg-white/5">
                        <div className="flex items-center gap-2 overflow-hidden">
                           <Code size={14} className="text-aether-indigo" />
                           <span className="text-xs font-medium text-white/60 truncate">{currentFile}</span>
                           {isModified && <div className="w-1.5 h-1.5 rounded-full bg-aether-indigo animate-pulse" />}
                        </div>
                        <div className="flex items-center gap-2">
                          <button 
                            onClick={handleRunCode}
                            className="flex items-center gap-1.5 text-[10px] font-bold uppercase tracking-widest px-3 py-1 rounded bg-white/5 text-white/60 hover:bg-white/10 transition-colors"
                          >
                            <Play size={10} className="fill-current" />
                            Run
                          </button>
                          <button 
                            onClick={handleFileSave}
                            disabled={!isModified}
                            className={`text-[10px] font-bold uppercase tracking-widest px-3 py-1 rounded transition-colors ${
                              isModified ? 'bg-aether-indigo text-white hover:bg-aether-indigo/80' : 'text-white/20'
                            }`}
                          >
                            Save
                          </button>
                        </div>
                      </div>
                      <div className="flex-1">
                        <MonacoEditor 
                          content={fileContent} 
                          language={currentFile.split('.').pop()} 
                          onChange={(val) => {
                            setFileContent(val);
                            setIsModified(true);
                          }}
                        />
                      </div>
                      <div className="h-48 border-t border-white/5 bg-black/40">
                         <TerminalView />
                      </div>
                    </>
                  ) : (
                    <div className="flex-1 flex items-center justify-center text-white/20 italic">
                      <div className="text-center">
                        <div className="w-20 h-20 rounded-3xl bg-white/5 flex items-center justify-center mx-auto mb-6">
                           <Code size={40} className="opacity-20" />
                        </div>
                        <p className="text-sm">Select a file from the explorer to begin editing</p>
                      </div>
                    </div>
                  )}
                </div>
              </motion.div>
            )}

            {activeTab === 'browser' && (
              <motion.div 
                key="browser"
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
                className="h-full"
              >
                <BrowserView />
              </motion.div>
            )}

            {activeTab === 'vision' && (
              <motion.div 
                key="vision"
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
                className="h-full"
              >
                <VisionGallery />
              </motion.div>
            )}

            {activeTab === 'workflows' && (
              <motion.div 
                key="workflows"
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
                className="h-full"
              >
                <WorkflowStudio />
              </motion.div>
            )}

            {activeTab === 'health' && (
              <motion.div 
                key="health"
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
                className="h-full"
              >
                <HealthDashboard />
              </motion.div>
            )}

            {activeTab === 'artifacts' && (
              <motion.div 
                key="artifacts"
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
                className="h-full"
              >
                <ArtifactExplorer />
              </motion.div>
            )}

            {activeTab === 'vault' && (
              <motion.div 
                key="vault"
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
                className="h-full"
              >
                <IdentityVault />
              </motion.div>
            )}

            {activeTab === 'plugins' && (
              <motion.div 
                key="plugins"
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
                className="h-full"
              >
                <PluginManager />
              </motion.div>
            )}

            {activeTab === 'audit' && (
              <motion.div 
                key="audit"
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
                className="h-full"
              >
                <AuditReplay />
              </motion.div>
            )}

            {activeTab === 'orchestration' && (
              <motion.div key="orchestration" initial={{ opacity: 0, x: 20 }} animate={{ opacity: 1, x: 0 }} exit={{ opacity: 0, x: -20 }} className="h-full">
                <OrchestrationMap />
              </motion.div>
            )}
            {activeTab === 'memory' && (
              <motion.div key="memory" initial={{ opacity: 0, x: 20 }} animate={{ opacity: 1, x: 0 }} exit={{ opacity: 0, x: -20 }} className="h-full">
                <MemoryExplorer />
              </motion.div>
            )}
          </AnimatePresence>
        </div>
      </main>

      <SettingsPanel 
        isOpen={isSettingsOpen} 
        onClose={() => setIsSettingsOpen(false)} 
        onSave={handleSaveSettings} 
      />

      <CommandPalette 
        isOpen={isCommandPaletteOpen} 
        onClose={() => setIsCommandPaletteOpen(false)} 
        onSelect={(id) => {
          if (id === 'settings') setIsSettingsOpen(true);
          if (id === 'new-workflow') setActiveTab('workflows');
          // Add more mappings as needed
        }}
      />
    </div>
  );
};

export default App;
