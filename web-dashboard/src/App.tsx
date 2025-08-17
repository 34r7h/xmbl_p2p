import React, { useState, useEffect } from 'react';
import Dashboard from './components/Dashboard';
import NodeManagement from './components/NodeManagement';
import StorageMonitor from './components/StorageMonitor';
import ComputeMonitor from './components/ComputeMonitor';
import BlockchainViewer from './components/BlockchainViewer';
import './App.css';

function App() {
  const [activeTab, setActiveTab] = useState('dashboard');
  const [systemStatus, setSystemStatus] = useState({
    monitoring: 'unknown',
    dashboard: 'unknown',
    simulator: 'unknown'
  });

  // Check system status on component mount
  useEffect(() => {
    checkSystemStatus();
    const interval = setInterval(checkSystemStatus, 10000); // Check every 10 seconds
    return () => clearInterval(interval);
  }, []);

  const checkSystemStatus = async () => {
    try {
      // Check monitoring service
      const monitoringResponse = await fetch('http://localhost:3001/health');
      setSystemStatus(prev => ({
        ...prev,
        monitoring: monitoringResponse.ok ? 'healthy' : 'unhealthy'
      }));
    } catch (error) {
      setSystemStatus(prev => ({
        ...prev,
        monitoring: 'unhealthy'
      }));
    }

    try {
      // Check dashboard
      const dashboardResponse = await fetch('http://localhost:3000');
      setSystemStatus(prev => ({
        ...prev,
        dashboard: dashboardResponse.ok ? 'healthy' : 'unhealthy'
      }));
    } catch (error) {
      setSystemStatus(prev => ({
        ...prev,
        dashboard: 'unhealthy'
      }));
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'healthy': return '#4CAF50';
      case 'unhealthy': return '#F44336';
      default: return '#FF9800';
    }
  };

  const getStatusText = (status: string) => {
    switch (status) {
      case 'healthy': return '✅ Healthy';
      case 'unhealthy': return '❌ Unhealthy';
      default: return '⚠️ Unknown';
    }
  };

  return (
    <div className="App">
      <header className="app-header">
        <h1>🚀 XMBL P2P Network Dashboard</h1>
        <div className="system-status">
          <div className="status-item">
            <span className="status-label">Monitoring:</span>
            <span className="status-value" style={{ color: getStatusColor(systemStatus.monitoring) }}>
              {getStatusText(systemStatus.monitoring)}
            </span>
          </div>
          <div className="status-item">
            <span className="status-label">Dashboard:</span>
            <span className="status-value" style={{ color: getStatusColor(systemStatus.dashboard) }}>
              {getStatusText(systemStatus.dashboard)}
            </span>
          </div>
        </div>
      </header>

      <nav className="app-nav">
        <button 
          className={`nav-button ${activeTab === 'dashboard' ? 'active' : ''}`}
          onClick={() => setActiveTab('dashboard')}
        >
          📊 Dashboard
        </button>
        <button 
          className={`nav-button ${activeTab === 'nodes' ? 'active' : ''}`}
          onClick={() => setActiveTab('nodes')}
        >
          🌐 Node Management
        </button>
        <button 
          className={`nav-button ${activeTab === 'storage' ? 'active' : ''}`}
          onClick={() => setActiveTab('storage')}
        >
          💾 Storage Monitor
        </button>
        <button 
          className={`nav-button ${activeTab === 'compute' ? 'active' : ''}`}
          onClick={() => setActiveTab('compute')}
        >
          ⚡ Compute Monitor
        </button>
        <button 
          className={`nav-button ${activeTab === 'blockchain' ? 'active' : ''}`}
          onClick={() => setActiveTab('blockchain')}
        >
          💰 Blockchain Viewer
        </button>
      </nav>

      <main className="app-main">
        {activeTab === 'dashboard' && <Dashboard />}
        {activeTab === 'nodes' && <NodeManagement />}
        {activeTab === 'storage' && <StorageMonitor />}
        {activeTab === 'compute' && <ComputeMonitor />}
        {activeTab === 'blockchain' && <BlockchainViewer />}
      </main>

      <footer className="app-footer">
        <p>XMBL P2P Storage and Compute Network - Complete System Dashboard</p>
        <div className="footer-links">
          <a href="http://localhost:3000/client.html" target="_blank" rel="noopener noreferrer">
            🚀 P2P Client App
          </a>
          <a href="http://localhost:3001" target="_blank" rel="noopener noreferrer">
            📊 Monitoring API
          </a>
        </div>
      </footer>
    </div>
  );
}

export default App;
