@ui @keyboard
Feature: Keyboard Shortcuts
  As an audiobook listener
  I want to control the application using keyboard shortcuts
  So that I can navigate and control playback efficiently

  # ==================== Playback Control ====================

  @critical
  Scenario: Space key toggles play/pause when file is loaded
    Given the application is running
    And a file is loaded for keyboard testing
    When I press Space key
    Then playback should start

  Scenario: Space key pauses when playing
    Given the application is running
    And a file is playing for keyboard testing
    When I press Space key
    Then playback should pause

  Scenario: M key toggles mute
    Given the application is running
    And a file is playing for keyboard testing
    When I press M key
    Then mute should toggle state

  Scenario: M key mutes audio
    Given the application is running
    And a file is playing with volume
    When I press M key
    Then audio should be muted

  Scenario: M key unmutes audio when muted
    Given the application is running
    And a file is playing with volume
    And I press M key
    When I press M key
    Then audio should be unmuted

  # ==================== Seek Controls ====================

  Scenario: ArrowRight seeks forward
    Given the application is running
    And a file is playing for keyboard testing
    When I press ArrowRight key
    Then the position should advance by skip duration

  Scenario: ArrowLeft seeks backward
    Given the application is running
    And a file is playing for keyboard testing
    When I press ArrowLeft key
    Then the position should rewind by skip duration

  Scenario: Shift+ArrowRight seeks larger amount
    Given the application is running
    And a file is playing for keyboard testing
    When I hold Shift and press ArrowRight
    Then the position should advance by skip duration

  Scenario: Shift+ArrowLeft seeks larger amount backward
    Given the application is running
    And a file is playing for keyboard testing
    When I hold Shift and press ArrowLeft
    Then the position should rewind by skip duration

  Scenario: Up arrow increases volume
    Given the application is running
    And a file is loaded for keyboard testing
    When I press Up arrow key
    Then volume should increase

  Scenario: Down arrow decreases volume
    Given the application is running
    And a file is playing with volume
    When I press Down arrow key
    Then volume should decrease

  # ==================== Modal Controls ====================

  Scenario: Escape closes open modal
    Given the application is running
    And a modal is open for keyboard testing
    When I press Escape key
    Then the open modal should close

  Scenario: Ctrl+Comma opens settings
    Given the application is running
    When I press Ctrl+Comma keys
    Then the settings modal should open via shortcut

  Scenario: Ctrl+T opens sleep timer
    Given the application is running
    When I press Ctrl+T keys
    Then the sleep timer modal should open via shortcut

  Scenario: F1 opens keyboard help
    Given the application is running
    When I press F1 key
    Then the keyboard help modal should open via shortcut

  # ==================== Bookmark Shortcuts ====================

  Scenario: Ctrl+B creates bookmark
    Given the application is running
    And a file is playing for keyboard testing
    When I press Ctrl+B keys
    Then a bookmark should be created via shortcut

  # ==================== Selection Shortcuts ====================

  Scenario: Ctrl+A selects all audiobooks
    Given the application is running
    And the application has 5 audiobooks
    When I press Ctrl+A keys
    Then all audiobooks should be selected via shortcut

  # ==================== Volume Control via Plus/Minus Keys ====================

  Scenario: Plus key increases volume by 5
    Given the application is running
    And a file is loaded with volume at 50
    When I press Plus key
    Then volume should increase by 5

  Scenario: Minus key decreases volume by 5
    Given the application is running
    And a file is loaded with volume at 50
    When I press Minus key
    Then volume should decrease by 5

  Scenario: Volume cannot exceed 100 with Plus key
    Given the application is running
    And a file is loaded with volume at 98
    When I press Plus key
    And I press Plus key
    Then volume should be at maximum 100

  Scenario: Volume cannot go below 0 with Minus key
    Given the application is running
    And a file is loaded with volume at 2
    When I press Minus key
    And I press Minus key
    Then volume should be at minimum 0

  Scenario: Equals key also increases volume
    Given the application is running
    And a file is loaded with volume at 50
    When I press Equals key
    Then volume should increase by 5

  Scenario: Underscore key also decreases volume
    Given the application is running
    And a file is loaded with volume at 50
    When I press Underscore key
    Then volume should decrease by 5

  # ==================== File Navigation via Arrow Keys ====================

  Scenario: ArrowDown navigates to next file
    Given the application is running
    And an audiobook with 3 files is loaded
    And the first file is selected
    When I press ArrowDown key for file navigation
    Then the second file should be selected

  Scenario: ArrowUp navigates to previous file
    Given the application is running
    And an audiobook with 3 files is loaded
    And the second file is selected
    When I press ArrowUp key for file navigation
    Then the first file should be selected

  Scenario: ArrowUp at first file does nothing
    Given the application is running
    And an audiobook with 3 files is loaded
    And the first file is selected
    When I press ArrowUp key for file navigation
    Then the first file should remain selected

  Scenario: ArrowDown at last file does nothing
    Given the application is running
    And an audiobook with 3 files is loaded
    And the third file is selected
    When I press ArrowDown key for file navigation
    Then the third file should remain selected

  # ==================== Focus Navigation ====================

  Scenario: Tab moves focus forward
    Given the application is running
    When I press Tab key
    Then focus should move to next element

  Scenario: Shift+Tab moves focus backward
    Given the application is running
    And I press Tab key
    When I press Shift+Tab keys
    Then focus should move to previous element

  Scenario: Enter activates focused element
    Given the application is running
    And a file is loaded for keyboard testing
    When I press Tab key
    And I press Enter key
    Then the focused element should be activated
