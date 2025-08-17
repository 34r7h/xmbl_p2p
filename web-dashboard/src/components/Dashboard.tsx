import React, { useState, useEffect } from 'react';

interface NetworkMetrics {
  totalNodes: number;
  onlineNodes: number;
  storageUsed: number;
  storageTotal: number;
  activeTasks: number;
  completedTasks: number;
  networkHealth: number;
  avgLatency: number;
}

const Dashboard: React.FC = () => {
  const [metrics, setMetrics] = useState<NetworkMetrics>({
    totalNodes: 10,
    onlineNodes: 10,
    storageUsed: 500,
    storageTotal: 1000,
    activeTasks: 3,
    completedTasks: 150,
    networkHealth: 95,
    avgLatency: 15
  });

  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Simulate loading real data
    const timer = setTimeout(() => {
      setLoading(false);
    }, 1000);

    return () => clearTimeout(timer);
  }, []);

  const getStoragePercentage = () => {
    return (metrics.storageUsed / metrics.storageTotal) * 100;
  };

  const getHealthColor = (health: number) => {
    if (health >= 90) return '#4CAF50';
    if (health >= 70) return '#FF9800';
    return '#F44336';
  };

  if (loading) {
    return (
      <div className="dashboard-grid">
        <div className="dashboard-card">
          <div className="loading-spinner"></div>
          <p style={{ textAlign: 'center' }}>Loading network metrics...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="fade-in">
      <h2 style={{ marginBottom: '30px', color: '#ffd700', fontSize: '2em' }}>
        📊 Network Overview Dashboard
      </h2>
      
      <div className="dashboard-grid">
        {/* Network Status Card */}
        <div className="dashboard-card">
          <h3>🌐 Network Status</h3>
          <div className="metric-item">
            <span className="metric-label">Total Nodes:</span>
            <span className="metric-value">{metrics.totalNodes}</span>
          </div>
          <div className="metric-item">
            <span className="metric-label">Online Nodes:</span>
            <span className="metric-value">
              <span className="status-indicator status-online"></span>
              {metrics.onlineNodes}
            </span>
          </div>
          <div className="metric-item">
            <span className="metric-label">Network Health:</span>
            <span className="metric-value" style={{ color: getHealthColor(metrics.networkHealth) }}>
              {metrics.networkHealth}%
            </span>
          </div>
          <div className="metric-item">
            <span className="metric-label">Avg Latency:</span>
            <span className="metric-value">{metrics.avgLatency}ms</span>
          </div>
        </div>

        {/* Storage Status Card */}
        <div className="dashboard-card">
          <h3>💾 Storage Status</h3>
          <div className="metric-item">
            <span className="metric-label">Storage Used:</span>
            <span className="metric-value">{metrics.storageUsed}GB</span>
          </div>
          <div className="metric-item">
            <span className="metric-label">Storage Total:</span>
            <span className="metric-value">{metrics.storageTotal}GB</span>
          </div>
          <div className="metric-item">
            <span className="metric-label">Usage:</span>
            <span className="metric-value">{getStoragePercentage().toFixed(1)}%</span>
          </div>
          <div className="progress-container">
            <div className="progress-bar">
              <div 
                className="progress-fill" 
                style={{ width: `${getStoragePercentage()}%` }}
              ></div>
            </div>
          </div>
        </div>

        {/* Compute Status Card */}
        <div className="dashboard-card">
          <h3>⚡ Compute Status</h3>
          <div className="metric-item">
            <span className="metric-label">Active Tasks:</span>
            <span className="metric-value">{metrics.activeTasks}</span>
          </div>
          <div className="metric-item">
            <span className="metric-label">Completed Tasks:</span>
            <span className="metric-value">{metrics.completedTasks}</span>
          </div>
          <div className="metric-item">
            <span className="metric-label">Success Rate:</span>
            <span className="metric-value">98.5%</span>
          </div>
          <div className="metric-item">
            <span className="metric-label">Avg Task Time:</span>
            <span className="metric-value">45s</span>
          </div>
        </div>

        {/* Performance Metrics Card */}
        <div className="dashboard-card">
          <h3>📈 Performance Metrics</h3>
          <div className="metric-item">
            <span className="metric-label">CPU Usage:</span>
            <span className="metric-value">25%</span>
          </div>
          <div className="metric-item">
            <span className="metric-label">Memory Usage:</span>
            <span className="metric-value">45%</span>
          </div>
          <div className="metric-item">
            <span className="metric-label">Network I/O:</span>
            <span className="metric-value">1.2 GB/s</span>
          </div>
          <div className="metric-item">
            <span className="metric-label">Response Time:</span>
            <span className="metric-value">15ms</span>
          </div>
        </div>

        {/* Recent Activity Card */}
        <div className="dashboard-card">
          <h3>🔄 Recent Activity</h3>
          <div className="metric-item">
            <span className="metric-label">Last File Upload:</span>
            <span className="metric-value">2 min ago</span>
          </div>
          <div className="metric-item">
            <span className="metric-label">Last Compute Task:</span>
            <span className="metric-value">5 min ago</span>
          </div>
          <div className="metric-item">
            <span className="metric-label">New Node Joined:</span>
            <span className="metric-value">10 min ago</span>
          </div>
          <div className="metric-item">
            <span className="metric-label">System Update:</span>
            <span className="metric-value">1 hour ago</span>
          </div>
        </div>

        {/* Quick Actions Card */}
        <div className="dashboard-card">
          <h3>⚡ Quick Actions</h3>
          <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
            <button className="btn" onClick={() => window.open('http://localhost:3000/client.html', '_blank')}>
              🚀 Open P2P Client
            </button>
            <button className="btn btn-secondary" onClick={() => window.open('http://localhost:3001', '_blank')}>
              📊 View Monitoring API
            </button>
            <button className="btn btn-warning" onClick={() => alert('Refresh functionality coming soon!')}>
              🔄 Refresh Metrics
            </button>
          </div>
        </div>
      </div>

      {/* System Alerts */}
      <div style={{ marginTop: '30px' }}>
        <h3 style={{ color: '#ffd700', marginBottom: '20px' }}>🚨 System Alerts</h3>
        <div className="alert info">
          <strong>System Status:</strong> All services are running normally. Network health is excellent.
        </div>
        <div className="alert success">
          <strong>Performance:</strong> System performance is within optimal parameters.
        </div>
      </div>
    </div>
  );
};

export default Dashboard;
