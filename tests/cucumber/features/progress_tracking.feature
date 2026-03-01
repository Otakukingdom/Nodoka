Feature: Progress Tracking
  As an audiobook listener
  I want my progress to be tracked automatically
  So that I can resume listening where I left off

  Scenario: New audiobook has zero progress
    Given the application has an audiobook
    When I check the audiobook progress
    Then the completeness should be zero

  Scenario: Updating audiobook progress
    Given the application has an audiobook
    When I update the audiobook completeness
    Then the audiobook completeness should be updated

  Scenario: File progress is tracked
    Given the application has an audiobook
    And the audiobook has files
    When I update the file progress
    Then the file progress should be updated

  Scenario: Metadata persistence
    Given the application has a settings store
    When I set a metadata value
    Then the metadata value should be stored
