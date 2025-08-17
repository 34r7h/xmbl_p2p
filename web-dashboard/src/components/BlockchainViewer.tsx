import React from 'react';

const mockTransactions = [
  {
    id: "tx_1",
    from: "alice",
    to: "bob",
    amount: 100,
    status: "confirmed"
  }
];

const BlockchainViewer: React.FC = () => {
  return (
    <section>
      <h2>Blockchain Transaction Viewer</h2>
      <ul>
        {mockTransactions.map(tx => (
          <li key={tx.id}>
            <strong>{tx.id}</strong>: {tx.from} → {tx.to} | Amount: {tx.amount} | Status: {tx.status}
          </li>
        ))}
      </ul>
    </section>
  );
};

export default BlockchainViewer;
