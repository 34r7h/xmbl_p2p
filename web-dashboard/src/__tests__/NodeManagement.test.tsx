
import React from 'react';
import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import NodeManagement from '../components/NodeManagement';

describe('NodeManagement', () => {
  it('renders node info', () => {
    render(<NodeManagement />);
    expect(screen.getByText(/Node Management/i)).toBeInTheDocument();
    expect(screen.getByText(/node_1/i)).toBeInTheDocument();
  });
});
