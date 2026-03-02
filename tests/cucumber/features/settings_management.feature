@ui
Feature: Settings Management
  As an audiobook listener
  I want to customize my listening experience
  So that the application works according to my preferences

  @critical
  Scenario: Theme preference is persisted
    Given the application has a settings store
    When I set the theme to Dark
    Then the theme should be stored as Dark

  Scenario: Volume preference is persisted
    Given the application has a settings store
    When I set the volume to 75
    Then the volume should be stored as 75

  Scenario: Speed preference is persisted
    Given the application has a settings store
    When I set the speed to 1.5
    Then the speed should be stored as 1.5

  Scenario: Panel ratio is persisted
    Given the application has a settings store
    When I set the panel ratio to 0.5
    Then the panel ratio should be stored as 0.5

  Scenario: Focus mode can be enabled
    Given the application has a settings store
    When I enable focus mode
    Then focus mode should be enabled

  Scenario: Focus mode can be disabled
    Given the application has a settings store
    When I enable focus mode
    And I disable focus mode
    Then focus mode should be disabled

  Scenario: Reduced motion preference is persisted
    Given the application has a settings store
    When I enable prefers reduced motion
    Then the reduced motion preference should be stored as true

  Scenario: All settings can be retrieved
    Given the application has a settings store
    And I set the theme to Dark
    And I set the volume to 75
    When I retrieve settings
    Then all settings should be available

  # Extended scenarios for window, skip, and auto-save settings

  Scenario: Window position is persisted
    Given the application has a settings store
    When I set the window position to 100, 200
    Then the window position should be 100, 200

  Scenario: Window size is persisted
    Given the application has a settings store
    When I set the window size to 800, 600
    Then the window size should be 800, 600

  Scenario: Skip forward duration is persisted
    Given the application has a settings store
    When I set the skip forward duration to 30 seconds
    Then the skip forward duration should be 30

  Scenario: Skip backward duration is persisted
    Given the application has a settings store
    When I set the skip backward duration to 15 seconds
    Then the skip backward duration should be 15

  Scenario: Auto-save interval is persisted
    Given the application has a settings store
    When I set the auto-save interval to 10 seconds
    Then the auto-save interval should be 10

  Scenario: Auto-save interval rejects invalid values
    Given the application has a settings store
    When I try to set the auto-save interval to 0 seconds
    Then the operation should fail

  Scenario: Auto-save interval rejects values over 60
    Given the application has a settings store
    When I try to set the auto-save interval to 61 seconds
    Then the operation should fail

  Scenario: Current audiobook is persisted
    Given the application has a settings store
    When I set the current audiobook to 42
    Then the current audiobook should be 42

  Scenario: Current file is persisted
    Given the application has a settings store
    When I set the current file to "/path/to/file.mp3"
    Then the current file should be "/path/to/file.mp3"

  Scenario: Reset to defaults
    Given the application has a settings store
    And I set the volume to 50
    And I set the speed to 2.0
    When I reset settings to defaults
    Then the volume should be 100
    And the speed should be 1.0

  Scenario: Corrupted current audiobook is cleared
    Given the application has a settings store
    And the current audiobook setting is corrupted
    When I get the current audiobook
    Then the result should be None

  # Cross-session persistence scenarios

  @critical
  Scenario: Theme preference persists across restarts
    Given the application has a settings store
    And I set the theme to Dark
    When I restart the application
    Then the theme should still be Dark
    And the UI should display in dark mode

  Scenario: Volume level persists across sessions
    Given the application has a settings store
    And I set the volume to 75
    When I restart the application
    Then the volume should be 75
    And the volume slider should show 75

  Scenario: Speed preference persists across sessions
    Given the application has a settings store
    And I set the speed to 1.5
    When I restart the application
    Then the speed should be 1.5

  Scenario: Last played position is restored
    Given the application has an audiobook
    And I am playing an audiobook at position 5 minutes
    When I restart the application
    Then the audiobook should be selected
    And the position should be restored to 5 minutes

  Scenario: Skip duration preferences persist
    Given the application has a settings store
    And I set the skip forward duration to 30 seconds
    And I set the skip backward duration to 15 seconds
    When I restart the application
    Then the skip forward duration should be 30
    And the skip backward duration should be 15

  Scenario: Focus mode preference persists
    Given the application has a settings store
    And I enable focus mode
    When I restart the application
    Then focus mode should be enabled

  # ==================== Settings Modal UI Interaction Scenarios ====================

  Scenario: Theme dropdown selection via UI
    Given the settings modal is open
    When I click the theme dropdown
    And I select Dark theme option
    Then the theme should change to Dark immediately
    And the UI should update to dark mode

  Scenario: Theme dropdown has all options
    Given the settings modal is open
    When I view the theme dropdown options
    Then Light option should be available
    And Dark option should be available
    And System option should be available

  Scenario: Add directory button triggers file picker
    Given the settings modal is open
    When I click the add directory button
    Then a file picker dialog should be triggered

  Scenario: Remove directory button removes entry
    Given the settings modal is open
    And 2 directories have been added
    When I click remove on the first directory
    Then only 1 directory should remain in the list

  Scenario: Skip forward duration dropdown selection
    Given the settings modal is open
    When I expand the playback settings section
    And I select 60 seconds for skip forward duration
    Then the skip forward duration should be set to 60

  Scenario: Skip backward duration dropdown selection
    Given the settings modal is open
    When I expand the playback settings section
    And I select 15 seconds for skip backward duration
    Then the skip backward duration should be set to 15

  Scenario: Auto-save interval dropdown selection
    Given the settings modal is open
    When I expand the playback settings section
    And I select 30 seconds for auto-save interval
    Then the auto-save interval should be set to 30

  Scenario: Focus mode toggle via checkbox
    Given the settings modal is open
    When I expand the accessibility settings section
    And I check the focus mode checkbox
    Then focus mode should be enabled

  Scenario: Reduced motion toggle via checkbox
    Given the settings modal is open
    When I expand the accessibility settings section
    And I check the reduced motion checkbox
    Then reduced motion should be enabled

  Scenario: Reset to defaults button restores defaults
    Given the settings modal is open
    And I have changed multiple settings
    When I click the reset to defaults button
    Then all settings should return to default values
    And theme should be Light
    And focus mode should be disabled

  Scenario: Done button closes modal and saves
    Given the settings modal is open
    And I have changed the theme to Dark
    When I click the Done button
    Then the settings modal should close
    And the theme setting should be saved

  Scenario: Close button closes modal
    Given the settings modal is open
    When I click the close button in the modal header
    Then the settings modal should close

  Scenario: Escape key closes settings modal
    Given the settings modal is open
    When I press Escape key
    Then the settings modal should close

  Scenario: Playback settings section expands and collapses
    Given the settings modal is open
    When I click the playback settings header
    Then the playback settings section should expand
    When I click the playback settings header again
    Then the playback settings section should collapse

  Scenario: Accessibility settings section expands and collapses
    Given the settings modal is open
    When I click the accessibility settings header
    Then the accessibility settings section should expand
    When I click the accessibility settings header again
    Then the accessibility settings section should collapse

  Scenario: Directory rescan button triggers rescan
    Given the settings modal is open
    And a directory has been added
    When I click the rescan button for that directory
    Then a rescan should be triggered for that directory
    And a loading indicator should appear

  Scenario: Settings modal shows directory paths
    Given the settings modal is open
    And 2 directories have been added
    When I view the directories section
    Then each directory path should be displayed
    And each directory should have rescan and remove buttons
