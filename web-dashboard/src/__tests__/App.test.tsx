
import React from 'react';
import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import App from '../App';

describe('App', () => {
  it('renders the dashboard title', () => {
    render(<App />);
    expect(screen.getByText(/XMBL Network Web Dashboard/i)).toBeInTheDocument();
  });
});
