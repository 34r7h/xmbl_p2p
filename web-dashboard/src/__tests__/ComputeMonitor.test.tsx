
import React from 'react';
import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import ComputeMonitor from '../components/ComputeMonitor';

describe('ComputeMonitor', () => {
  it('shows compute metrics', () => {
    render(<ComputeMonitor />);
    expect(screen.getByText(/Compute Monitor/i)).toBeInTheDocument();
    expect(screen.getByText(/Active Tasks/i)).toBeInTheDocument();
  });
});
