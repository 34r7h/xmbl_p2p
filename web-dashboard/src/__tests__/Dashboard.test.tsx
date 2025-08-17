
import React from 'react';
import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import Dashboard from '../components/Dashboard';

describe('Dashboard', () => {
  it('shows network status', () => {
    render(<Dashboard />);
    expect(screen.getByText(/Network Status Dashboard/i)).toBeInTheDocument();
    expect(screen.getByText(/Online/i)).toBeInTheDocument();
  });
});
