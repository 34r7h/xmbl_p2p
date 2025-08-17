
import React from 'react';
import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import BlockchainViewer from '../components/BlockchainViewer';

describe('BlockchainViewer', () => {
  it('shows blockchain transactions', () => {
    render(<BlockchainViewer />);
    expect(screen.getByText(/Blockchain Transaction Viewer/i)).toBeInTheDocument();
    expect(screen.getByText(/tx_1/i)).toBeInTheDocument();
  });
});
