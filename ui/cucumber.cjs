/**
 * Cucumber configuration for Playwright-based UI acceptance tests
 * Integrates Cucumber with Playwright for end-to-end browser testing
 *
 * Run with: NODE_OPTIONS="--import tsx" npx cucumber-js --config cucumber.cjs
 */

module.exports = {
  default: {
    paths: ['../tests/cucumber/features/**/*.feature'],
    require: ['e2e/cucumber/steps/**/*.ts', 'e2e/cucumber/hooks.ts'],
    requireModule: ['tsx/cjs'],
    format: ['html:e2e/cucumber-report.html', 'progress'],
    parallel: 1,
    retry: 1,
    timeout: 60000,
    tags: '@ui',
  },
};
