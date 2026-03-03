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

  # ==================== UI Verification Scenarios ====================

  Scenario: Progress bar updates during playback
    Given the application has an audiobook with 3 files
    And I am playing the first file
    When I wait for 2 seconds of playback
    Then the seek bar position should have advanced
    And the progress bar should be visually updated

  Scenario: Progress percentage displayed on audiobook card
    Given the application has an audiobook at 50% completion
    When I view the library
    Then the audiobook card should show 50% progress
    And a progress bar should be visible on the card

  Scenario: Resume position indicator on audiobook
    Given the application has an audiobook with progress
    When I view the library
    Then the audiobook should show a resume indicator
    And the last played position should be displayed

  Scenario: Progress persists after page reload
    Given the application has an audiobook with 3 files
    And I am playing the first file
    When I wait for 2 seconds of playback
    And I pause playback
    And I reload the page
    Then the seek position should be restored
    And the progress should be displayed correctly

  Scenario: File progress indicator shows in file list
    Given the application has an audiobook with 3 files
    And the first file has 75% progress
    When I select the audiobook
    Then the first file should show 75% progress
    And a progress bar should be visible on the file item

  Scenario: Overall audiobook progress calculates correctly
    Given the application has an audiobook with 3 files
    And the first file has 100% progress
    And the second file has 50% progress
    And the third file has 0% progress
    When I view the audiobook details
    Then the overall progress should show 50%

  Scenario: Progress bar color changes with completion
    Given the application has an audiobook with 3 files
    When I complete the first file
    Then the file progress bar should show complete styling
    And the audiobook progress should update accordingly

  Scenario: Remaining time is calculated and displayed
    Given the application has an audiobook with 2 hours total duration
    And I am at 50% progress
    When I view the player controls
    Then the remaining time should show approximately 1 hour
    And the elapsed time should show approximately 1 hour
