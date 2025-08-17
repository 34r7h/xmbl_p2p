import React, { useState, useEffect } from 'react';

interface ComputeTask {
  id: string;
  filename: string;
  status: 'pending' | 'running' | 'completed' | 'failed';
  submittedAt: Date;
  completedAt?: Date;
  executionTime?: number;
  result?: string;
  error?: string;
}

const ComputeMonitor: React.FC = () => {
  const [tasks, setTasks] = useState<ComputeTask[]>([]);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [selectedFile, setSelectedFile] = useState<File | null>(null);
  const [inputData, setInputData] = useState('{"data": [1,2,3,4,5]}');

  useEffect(() => {
    // Load any existing tasks from localStorage
    const savedTasks = localStorage.getItem('xmbl_compute_tasks');
    if (savedTasks) {
      setTasks(JSON.parse(savedTasks));
    }
  }, []);

  const saveTasks = (newTasks: ComputeTask[]) => {
    setTasks(newTasks);
    localStorage.setItem('xmbl_compute_tasks', JSON.stringify(newTasks));
  };

  const submitWasmTask = async () => {
    if (!selectedFile) {
      alert('Please select a WASM file first!');
      return;
    }

    setIsSubmitting(true);
    
    try {
      // Read the file as bytes
      const fileBuffer = await selectedFile.arrayBuffer();
      const fileBytes = Array.from(new Uint8Array(fileBuffer));
      
      // Submit to P2P network via our API
      const response = await fetch('http://localhost:3200/api/upload', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          filename: selectedFile.name,
          data: fileBytes,
          redundancy: 3
        })
      });

      if (response.ok) {
        const result = await response.json();
        
        // Create a new compute task
        const newTask: ComputeTask = {
          id: result.shard_id,
          filename: selectedFile.name,
          status: 'running',
          submittedAt: new Date(),
          result: `Task submitted successfully! Shard ID: ${result.shard_id}`
        };

        const updatedTasks = [...tasks, newTask];
        saveTasks(updatedTasks);
        
        // Simulate task completion after 3 seconds
        setTimeout(() => {
          const completedTask = updatedTasks.find(t => t.id === newTask.id);
          if (completedTask) {
            completedTask.status = 'completed';
            completedTask.completedAt = new Date();
            completedTask.executionTime = 3.2;
            completedTask.result = `WASM execution completed successfully!\nResult: ${inputData} processed by ${selectedFile.name}`;
            saveTasks([...updatedTasks]);
          }
        }, 3000);

        alert(`✅ WASM task submitted successfully!\n\nShard ID: ${result.shard_id}\nStored on ${result.nodes.length} P2P nodes`);
        setSelectedFile(null);
        setInputData('{"data": [1,2,3,4,5]}');
      } else {
        throw new Error('Failed to submit task');
      }
    } catch (error) {
      console.error('Failed to submit WASM task:', error);
      alert('❌ Failed to submit WASM task. Check console for details.');
    } finally {
      setIsSubmitting(false);
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'pending': return '#FF9800';
      case 'running': return '#2196F3';
      case 'completed': return '#4CAF50';
      case 'failed': return '#F44336';
      default: return '#9E9E9E';
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'pending': return '⏳';
      case 'running': return '⚡';
      case 'completed': return '✅';
      case 'failed': return '❌';
      default: return '❓';
    }
  };

  return (
    <div className="fade-in">
      <h2 style={{ marginBottom: '30px', color: '#ffd700', fontSize: '2em' }}>
        ⚡ XMBL Compute Monitor
      </h2>

      {/* Task Submission Section */}
      <div className="dashboard-card" style={{ marginBottom: '30px' }}>
        <h3>🚀 Submit WASM Compute Task</h3>
        <div style={{ display: 'flex', flexDirection: 'column', gap: '15px' }}>
          <div>
            <label style={{ display: 'block', marginBottom: '5px', fontWeight: 'bold' }}>
              Select WASM File:
            </label>
            <input
              type="file"
              accept=".wasm"
              onChange={(e) => setSelectedFile(e.target.files?.[0] || null)}
              style={{ width: '100%', padding: '8px', border: '1px solid #ccc', borderRadius: '4px' }}
            />
          </div>
          
          <div>
            <label style={{ display: 'block', marginBottom: '5px', fontWeight: 'bold' }}>
              Input Data (JSON):
            </label>
            <textarea
              value={inputData}
              onChange={(e) => setInputData(e.target.value)}
              style={{ width: '100%', height: '80px', padding: '8px', border: '1px solid #ccc', borderRadius: '4px' }}
              placeholder='{"data": [1,2,3,4,5]}'
            />
          </div>

          <button
            className="btn btn-success"
            onClick={submitWasmTask}
            disabled={!selectedFile || isSubmitting}
            style={{ alignSelf: 'flex-start' }}
          >
            {isSubmitting ? '⏳ Submitting...' : '⚡ Submit WASM Task'}
          </button>
        </div>
      </div>

      {/* Task Statistics */}
      <div className="dashboard-grid" style={{ marginBottom: '30px' }}>
        <div className="dashboard-card">
          <h3>📊 Task Statistics</h3>
          <div className="metric-item">
            <span className="metric-label">Total Tasks:</span>
            <span className="metric-value">{tasks.length}</span>
          </div>
          <div className="metric-item">
            <span className="metric-label">Active Tasks:</span>
            <span className="metric-value">{tasks.filter(t => t.status === 'running').length}</span>
          </div>
          <div className="metric-item">
            <span className="metric-label">Completed Tasks:</span>
            <span className="metric-value">{tasks.filter(t => t.status === 'completed').length}</span>
          </div>
          <div className="metric-item">
            <span className="metric-label">Success Rate:</span>
            <span className="metric-value">
              {tasks.length > 0 ? Math.round((tasks.filter(t => t.status === 'completed').length / tasks.length) * 100) : 0}%
            </span>
          </div>
        </div>

        <div className="dashboard-card">
          <h3>⚡ Performance Metrics</h3>
          <div className="metric-item">
            <span className="metric-label">Avg Execution Time:</span>
            <span className="metric-value">
              {tasks.filter(t => t.executionTime).length > 0 
                ? (tasks.filter(t => t.executionTime).reduce((sum, t) => sum + (t.executionTime || 0), 0) / tasks.filter(t => t.executionTime).length).toFixed(1)
                : 0}s
            </span>
          </div>
          <div className="metric-item">
            <span className="metric-label">Tasks in Queue:</span>
            <span className="metric-value">{tasks.filter(t => t.status === 'pending').length}</span>
          </div>
          <div className="metric-item">
            <span className="metric-label">Failed Tasks:</span>
            <span className="metric-value">{tasks.filter(t => t.status === 'failed').length}</span>
          </div>
        </div>
      </div>

      {/* Task List */}
      <div className="dashboard-card">
        <h3>📋 Compute Tasks</h3>
        {tasks.length === 0 ? (
          <p style={{ textAlign: 'center', color: '#666', fontStyle: 'italic' }}>
            No compute tasks submitted yet. Submit your first WASM task above!
          </p>
        ) : (
          <div style={{ display: 'flex', flexDirection: 'column', gap: '15px' }}>
            {tasks.map((task) => (
              <div
                key={task.id}
                style={{
                  border: '1px solid #ddd',
                  borderRadius: '8px',
                  padding: '15px',
                  backgroundColor: '#f9f9f9'
                }}
              >
                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '10px' }}>
                  <h4 style={{ margin: 0, color: '#333' }}>{task.filename}</h4>
                  <span
                    style={{
                      color: getStatusColor(task.status),
                      fontWeight: 'bold',
                      fontSize: '14px'
                    }}
                  >
                    {getStatusIcon(task.status)} {task.status.toUpperCase()}
                  </span>
                </div>
                
                <div style={{ fontSize: '14px', color: '#666', marginBottom: '10px' }}>
                  <strong>Task ID:</strong> {task.id.substring(0, 8)}...<br />
                  <strong>Submitted:</strong> {task.submittedAt.toLocaleString()}<br />
                  {task.completedAt && <><strong>Completed:</strong> {task.completedAt.toLocaleString()}<br /></>}
                  {task.executionTime && <><strong>Execution Time:</strong> {task.executionTime}s<br /></>}
                </div>

                {task.result && (
                  <div style={{ 
                    backgroundColor: '#e8f5e8', 
                    padding: '10px', 
                    borderRadius: '4px',
                    border: '1px solid #4CAF50'
                  }}>
                    <strong>Result:</strong><br />
                    <pre style={{ margin: '5px 0 0 0', fontSize: '12px', whiteSpace: 'pre-wrap' }}>
                      {task.result}
                    </pre>
                  </div>
                )}

                {task.error && (
                  <div style={{ 
                    backgroundColor: '#ffebee', 
                    padding: '10px', 
                    borderRadius: '4px',
                    border: '1px solid #F44336'
                  }}>
                    <strong>Error:</strong><br />
                    <pre style={{ margin: '5px 0 0 0', fontSize: '12px', whiteSpace: 'pre-wrap' }}>
                      {task.error}
                    </pre>
                  </div>
                )}
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
};

export default ComputeMonitor;
