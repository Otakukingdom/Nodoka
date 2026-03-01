/**
 * Test setup for Vitest with React Testing Library
 */

import '@testing-library/jest-dom';
import * as React from 'react';

// Make React.act available globally for testing-library
(globalThis as { React?: typeof React }).React = React;
