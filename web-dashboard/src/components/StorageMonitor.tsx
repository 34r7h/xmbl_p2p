import React from 'react';

const StorageMonitor: React.FC = () => {
  return (
    <section>
      <h2>Storage Monitor</h2>
      <p>Used: <strong>500GB</strong></p>
      <p>Total: <strong>1000GB</strong></p>
      <p>Redundancy: <strong>2x</strong></p>
      <p>Availability: <strong>99.99%</strong></p>
    </section>
  );
};

export default StorageMonitor;
