/**
 * ESLint flat configuration for strict TypeScript linting
 */

import js from '@eslint/js';
import ts from 'typescript-eslint';

export default ts.config(
  js.configs.recommended,
  ...ts.configs.strict,
  ...ts.configs.stylistic,
  {
    ignores: [
      'dist/**',
      'node_modules/**',
      'src/**/*.test.ts',
      'src/**/*.test.tsx',
    ],
  },
  {
    languageOptions: {
      parserOptions: {
        project: true,
        tsconfigRootDir: import.meta.dirname,
      },
    },
    rules: {
      // Additional strict rules for maximum type safety
      '@typescript-eslint/no-floating-promises': 'error',
      '@typescript-eslint/no-misused-promises': 'error',
      '@typescript-eslint/require-await': 'error',
      '@typescript-eslint/no-unnecessary-condition': 'error',
      '@typescript-eslint/no-unnecessary-qualifier': 'error',
      '@typescript-eslint/no-unnecessary-type-arguments': 'error',
      '@typescript-eslint/prefer-nullish-coalescing': 'error',
      '@typescript-eslint/prefer-optional-chain': 'error',
      '@typescript-eslint/no-explicit-any': 'error',
      '@typescript-eslint/no-unsafe-assignment': 'error',
      '@typescript-eslint/no-unsafe-call': 'error',
      '@typescript-eslint/no-unsafe-member-access': 'error',
      '@typescript-eslint/no-unsafe-return': 'error',

      // Relax strict-boolean-expressions as it's too strict for many valid patterns
      '@typescript-eslint/strict-boolean-expressions': 'off',

      // Allow non-null assertions in certain cases
      '@typescript-eslint/no-non-null-assertion': 'warn',

      // Consistent generic constructors
      '@typescript-eslint/consistent-generic-constructors': 'error',

      // Array type style
      '@typescript-eslint/array-type': ['error', { default: 'array-simple' }],
    },
  }
);
