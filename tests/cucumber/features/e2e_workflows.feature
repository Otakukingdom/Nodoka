@ui @e2e
Feature: Complete User Workflows
  As an audiobook listener
  I want the application to work seamlessly end-to-end
  So that I can enjoy my audiobooks without issues

  # ==================== Complete Playback Workflow ====================

  @critical
  Scenario: Complete playback workflow from selection to bookmark
    Given the application has an audiobook with 3 files
    When I select the audiobook from the library
    And I double-click the first file to play
    And I wait for 2 seconds of playback
    And I press Space key to pause
    And I press ArrowRight key to seek forward
    And I press Ctrl+B keys to create a bookmark
    And I click the stop button
    Then the bookmark should be saved
    And the progress should be persisted
    And the audiobook should show as "in progress"

  Scenario: Cross-file playback continues seamlessly
    Given the application has an audiobook with 3 files
    And I am playing the first file near the end
    When the first file ends
    Then the second file should start playing automatically
    And the UI should update to show the second file selected
    And playback should continue without interruption

  Scenario: Resume playback from last position
    Given the application has an audiobook with 3 files
    And I have played the first file to 50% progress
    And I have closed the application
    When I restart the application
    And I select the audiobook from the library
    Then the resume position should be displayed
    When I click the resume button
    Then playback should start from the saved position

  # ==================== Library Management Workflow ====================

  Scenario: Add directory and scan for audiobooks
    Given the application is running
    When I open the settings modal
    And I add a directory to the library
    And I wait for scanning to complete
    Then audiobooks should appear in the library
    And a scan complete notification should appear

  Scenario: Manage audiobook metadata
    Given the application has an audiobook
    When I select the audiobook from the library
    And I edit the audiobook author to "New Author"
    And I edit the audiobook title to "New Title"
    Then the changes should be saved
    And the updated metadata should be displayed

  Scenario: Delete audiobook with confirmation
    Given the application has 3 audiobooks
    When I right-click on the first audiobook
    And I select delete from context menu
    Then a confirmation dialog should appear
    When I confirm the deletion
    Then the audiobook should be deleted
    And 2 audiobooks should remain in the library

  # ==================== Bookmark Management Workflow ====================

  Scenario: Create and manage bookmarks during playback
    Given the application has an audiobook with 3 files
    And I am playing the first file
    When I create a bookmark with note "Chapter 1"
    And I seek to 5 minutes
    And I create a bookmark with note "Important part"
    Then 2 bookmarks should be saved
    When I click on the first bookmark
    Then playback should jump to the bookmark position

  # ==================== Settings Workflow ====================

  Scenario: Configure and persist user preferences
    Given the application is running
    When I open the settings modal
    And I change the theme preference to "dark"
    And I set skip forward duration to 60 seconds
    And I close the settings modal
    Then the dark theme should be applied immediately
    When I restart the application
    Then the dark theme should still be applied
    And the skip duration should be 60 seconds

  # ==================== Search and Filter Workflow ====================

  Scenario: Search and filter audiobooks
    Given the application has 5 audiobooks
    When I search for "Adventure"
    Then only matching audiobooks should be displayed
    When I clear the search
    Then all 5 audiobooks should be displayed
    When I sort by last listened
    Then audiobooks should be ordered by last played date

  # ==================== Bulk Operations Workflow ====================

  Scenario: Bulk select and delete audiobooks
    Given the application has 5 audiobooks
    When I select multiple audiobooks with Ctrl+Click
    And the bulk action bar should appear
    And I click bulk delete
    Then a confirmation dialog should appear
    When I confirm the deletion
    Then the selected audiobooks should be deleted

  Scenario: Bulk mark as played
    Given the application has 5 audiobooks
    And I select multiple audiobooks with Ctrl+Click
    When I click bulk mark as played
    Then all selected audiobooks should be marked as complete
    And a success notification should appear

  # ==================== Error Recovery Workflow ====================

  Scenario: Handle and recover from playback error
    Given the application has an audiobook with corrupted file
    When I try to play the corrupted file
    Then an error notification should appear
    And the error should include helpful information
    When I dismiss the error
    Then I should be able to select another file
    And playback should work for valid files

  # ==================== Keyboard Navigation Workflow ====================

  Scenario: Navigate application entirely with keyboard
    Given the application is running
    And a file is loaded for keyboard testing
    When I press Tab key repeatedly
    Then focus should move through all interactive elements
    When I press Space key
    Then playback should toggle
    When I press ArrowRight key
    Then the position should advance by skip duration
    When I press M key
    Then mute should toggle state
    When I press Escape key
    Then any open modal should close

  # ==================== Notification Workflow ====================

  Scenario: Multiple notifications queue and dismiss
    Given the application is running
    When I trigger 3 notifications in sequence
    Then 3 notifications should be visible
    And they should be stacked vertically
    When I click dismiss all notifications
    Then all notifications should disappear
