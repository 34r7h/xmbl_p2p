import React from 'react';

const mockNodes = [
  {
    id: "node_1",
    status: "online",
    storage: { used: 500, total: 1000 },
    compute: { active: 3, completed: 150 }
  }
];

const NodeManagement: React.FC = () => {
  return (
    <section>
      <h2>Node Management</h2>
      <ul>
        {mockNodes.map(node => (
          <li key={node.id}>
            <strong>{node.id}</strong> - Status: {node.status} | Storage: {node.storage.used}/{node.storage.total}GB | Compute: {node.compute.active} active, {node.compute.completed} completed
          </li>
        ))}
      </ul>
    </section>
  );
};

export default NodeManagement;
