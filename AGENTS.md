<!-- EXCEPTIONAL CASE: This file contains development agent instructions and is kept
     external as it serves AI coding assistants, not end users. This is NOT user-facing
     documentation and therefore qualifies as an exception to the "rustdoc only" rule. -->

No dead code allowed
Must have strict rust linting
No lint exceptions allowed
No documentation outside of rustdoc, rustdoc must stand on its own
NO TEMPORARY .md FILES EVER!
No files can be longer than 1000 lines. They must be refactored if any source code is longer than 1000 lines long. DO NOT USE names like part1/part2, etc. naming scheme. Everything must be named semantically
* Limited documentation can exist outside of rustdoc, but they must be EXCEPTIONAL circumstances, DO NOT create any external documentation without any strong justification. Documentation that exist outside of rustdoc MUST BE STRICTLY strutinized and err on the side of not having them
Unit and integration test must exist for all major features, and strictly test for behavior not implementation. If test fail, either the test behavior is wrong and must be updated, or the implementation is faulty. Do not change test unless behavior that is tested is wrong, or the test itself is buggy.
If test-driven-development skill exists, USE IT
