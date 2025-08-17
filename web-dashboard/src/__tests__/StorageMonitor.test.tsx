
import React from 'react';
import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import StorageMonitor from '../components/StorageMonitor';

describe('StorageMonitor', () => {
  it('shows storage usage', () => {
    render(<StorageMonitor />);
    expect(screen.getByText(/Storage Monitor/i)).toBeInTheDocument();
    expect(screen.getByText(/500GB/i)).toBeInTheDocument();
  });
});
