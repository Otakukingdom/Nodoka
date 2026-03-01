/**
 * Test setup for Vitest with React Testing Library
 */

import '@testing-library/jest-dom';
import * as React from 'react';
import { act } from 'react';

// Make React.act available globally for testing-library (React 19 compatibility)
// testing-library expects globalThis.React.act
const ReactWithAct = {
  ...React,
  act,
};

(globalThis as { React?: typeof ReactWithAct }).React = ReactWithAct;
