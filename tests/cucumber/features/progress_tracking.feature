@ui
Feature: Progress Tracking
  As an audiobook listener
  I want my progress to be tracked automatically
  So that I can resume listening where I left off

  @critical
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

  # Extended scenarios for file-level progress tracking

  Scenario: File seek position is updated
    Given the application has an audiobook
    And the audiobook has a file
    When I set the file seek position to 30000 ms
    Then the file seek position should be 30000

  Scenario: File completeness is tracked
    Given the application has an audiobook
    And the audiobook has a file
    When I set the file completeness to 50
    Then the file completeness should be 50

  Scenario: Last listened timestamp is updated
    Given the application has an audiobook
    When I update the last listened timestamp
    Then the last listened timestamp should be recent

  Scenario: Selected file is tracked
    Given the application has an audiobook
    And the audiobook has files
    When I select a file for playback
    Then the selected file should be stored
