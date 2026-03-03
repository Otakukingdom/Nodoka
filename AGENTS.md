<!-- EXCEPTIONAL CASE: This file contains development agent instructions and is kept
     external as it serves AI coding assistants, not end users. This is NOT user-facing
     documentation and therefore qualifies as an exception to the "rustdoc only" rule. -->


* No dead code allowed
* Must have strict rust linting
* No lint exceptions allowed
* No documentation outside of rustdoc, rustdoc must stand on its own
* NO TEMPORARY .md FILES EVER!
* No files can be longer than 1000 lines. They must be refactored if any source code is longer than 1000 lines long. DO NOT USE names like part1/part2, etc. naming scheme. Everything must be named semantically
* Limited documentation can exist outside of rustdoc, but they must be EXCEPTIONAL circumstances, DO NOT create any external documentation without any strong justification. Documentation that exist outside of rustdoc MUST BE STRICTLY strutinized and err on the side of not having them

# Testing
Unit and integration test must exist for all major features, and strictly test for behavior not implementation. If test fail, either the test behavior is wrong and must be updated, or the implementation is faulty. Do not change test unless behavior that is tested is wrong, or the test itself is buggy.
* If test-driven-development skill exists, USE IT
* All tests **must** be automated. No manual testing should exist

# UI Work

When performing UI-related work (modifying components, changing layouts, updating themes):

1. **Consult Screenshot Tests**: Review `xtask/src/screenshot/mod.rs` rustdoc and `ui/e2e/screenshot-generation.spec.ts` to understand current UI state coverage.

2. **Update Screenshot Tests**: If your UI changes affect any captured state:
   - Update the relevant test in `ui/e2e/screenshot-generation.spec.ts`
   - Add new test cases for new UI states
   - Run `cargo xtask screenshot` to regenerate screenshots

3. **Mock Data**: Add new mock data to `ui/e2e/fixtures/screenshot-scenarios.ts` if testing new data scenarios.

4. **Helpers**: Add state manipulation helpers to `ui/e2e/screenshot-helpers.ts` for complex UI state setup.

5. **Verification**: After UI changes, run `cargo xtask screenshot` and verify all screenshots are generated correctly.
