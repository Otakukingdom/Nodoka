No dead code allowed
Must have strict rust linting
No lint exceptions allowed
No documentation outside of rustdoc, rustdoc must stand on its own
* Limited documentation can exist outside of rustdoc, but they must be EXCEPTIONAL circumstances, DO NOT create any external documentation without any strong justification. Documentation that exist outside of rustdoc MUST BE STRICTLY strutinized and err on the side of not having them
Unit and integration test must exist for all major features, and strictly test for behavior not implementation. If test fail, either the test behavior is wrong and must be updated, or the implementation is faulty. Do not change test unless behavior that is tested is wrong, or the test itself is buggy.
If test-driven-development skill exists, USE IT
