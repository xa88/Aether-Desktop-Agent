import React, { useState, useEffect } from 'react';
import { ChevronRight, ChevronDown, File, Folder, Code } from 'lucide-react';

const FileTree = ({ onFileSelect }) => {
  const [files, setFiles] = useState([]);
  const [expanded, setExpanded] = useState({});

  useEffect(() => {
    const fetchFiles = async () => {
      if (window.ada && window.ada.listFiles) {
        const fileList = await window.ada.listFiles('');
        setFiles(fileList);
      }
    };
    fetchFiles();
  }, []);

  const toggleExpand = (path) => {
    setExpanded(prev => ({ ...prev, [path]: !prev[path] }));
  };

  const renderItem = (item) => {
    const isExpanded = expanded[item.path];
    const isDir = item.type === 'directory';

    return (
      <div key={item.path} className="select-none">
        <div 
          className={`flex items-center gap-2 p-1.5 rounded-lg hover:bg-white/5 cursor-pointer text-xs transition-colors group ${isDir ? 'text-white/80' : 'text-white/60'}`}
          onClick={() => isDir ? toggleExpand(item.path) : onFileSelect(item.path)}
        >
          {isDir ? (
            isExpanded ? <ChevronDown size={14} className="text-white/20" /> : <ChevronRight size={14} className="text-white/20" />
          ) : (
            <div className="w-[14px]" /> 
          )}
          
          {isDir ? (
            <Folder size={14} className={`${isExpanded ? 'text-aether-indigo' : 'text-white/40'} fill-current opacity-40`} />
          ) : (
            <File size={14} className="text-white/20" />
          )}
          
          <span className="truncate group-hover:text-white transition-colors">
            {item.name}
          </span>
        </div>

        {isDir && isExpanded && item.children && (
          <div className="ml-4 border-l border-white/5 pl-2 mt-0.5 space-y-0.5">
            {item.children.map(child => renderItem(child))}
          </div>
        )}
      </div>
    );
  };

  return (
    <div className="flex flex-col h-full overflow-y-auto custom-scrollbar p-2">
      <div className="text-[10px] font-bold text-white/30 uppercase tracking-[0.2em] mb-4 px-2">Workspace</div>
      <div className="space-y-0.5">
        {files.map(file => renderItem(file))}
      </div>
    </div>
  );
};

export default FileTree;
