@ui
Feature: Sleep Timer
  As an audiobook listener
  I want to set a sleep timer
  So that playback stops automatically when I fall asleep

  @critical
  Scenario: Creating a duration-based sleep timer
    Given I want to set a sleep timer
    When I set a sleep timer for 30 minutes
    Then the sleep timer should be active
    And the timer mode should be Duration

  Scenario: Creating an end-of-chapter sleep timer
    Given I want to set a sleep timer
    When I set a sleep timer for end of chapter
    Then the sleep timer should be active
    And the timer mode should be EndOfChapter

  Scenario: Checking if duration timer has expired
    Given I have a sleep timer set for 0 seconds
    When I check if the timer has expired
    Then the timer should be expired

  Scenario: Getting remaining time for duration timer
    Given I have a sleep timer set for 60 seconds
    When I check the remaining time
    Then the remaining time should be greater than zero

  Scenario: End-of-chapter timer has no remaining time
    Given I have an end-of-chapter sleep timer
    When I check the remaining time
    Then the remaining time should be None

  Scenario: Sleep timer with fade duration
    Given I want to set a sleep timer
    When I set a sleep timer for 30 minutes with 10 second fade
    Then the fade duration should be 10 seconds

  # ==================== Sleep Timer Modal UI Interaction Scenarios ====================

  Scenario: Sleep timer modal opens via button click
    Given the application is running
    When I click the sleep timer button in player controls
    Then the sleep timer modal should open

  Scenario: Sleep timer modal opens via keyboard shortcut
    Given the application is running
    When I press Ctrl+T keys
    Then the sleep timer modal should open

  Scenario: Sleep timer modal displays duration options
    Given the sleep timer modal is open
    When I view the duration options
    Then 15 minute option should be visible
    And 30 minute option should be visible
    And 60 minute option should be visible
    And 90 minute option should be visible

  Scenario: Sleep timer modal displays end of chapter option
    Given the sleep timer modal is open
    When I view the options
    Then the end of chapter option should be visible

  Scenario: Sleep timer modal displays fade duration dropdown
    Given the sleep timer modal is open
    When I view the fade options
    Then a fade duration dropdown should be visible
    And None option should be available
    And 5 seconds option should be available
    And 10 seconds option should be available
    And 15 seconds option should be available

  Scenario: Selecting 15 minute duration starts timer
    Given the sleep timer modal is open
    When I click the 15 minute duration button
    Then the modal should close
    And the sleep timer should be active
    And the remaining time should be approximately 15 minutes

  Scenario: Selecting 30 minute duration starts timer
    Given the sleep timer modal is open
    When I click the 30 minute duration button
    Then the modal should close
    And the sleep timer should be active

  Scenario: Selecting 60 minute duration starts timer
    Given the sleep timer modal is open
    When I click the 60 minute duration button
    Then the modal should close
    And the sleep timer should be active

  Scenario: Selecting 90 minute duration starts timer
    Given the sleep timer modal is open
    When I click the 90 minute duration button
    Then the modal should close
    And the sleep timer should be active

  Scenario: Selecting end of chapter starts timer
    Given the sleep timer modal is open
    When I click the end of chapter button
    Then the modal should close
    And the sleep timer should be active
    And the timer mode should be EndOfChapter

  Scenario: Selecting fade duration updates setting
    Given the sleep timer modal is open
    When I select 10 seconds from the fade dropdown
    Then the fade duration should be 10 seconds

  Scenario: Cancel timer button stops active timer
    Given a sleep timer is active
    When I open the sleep timer modal
    And I click the cancel timer button
    Then the sleep timer should be cancelled
    And no remaining time should be displayed

  Scenario: Active timer shows remaining time in modal
    Given a sleep timer is active with 30 minutes remaining
    When I open the sleep timer modal
    Then the remaining time should be displayed in the modal
    And the timer mode should be shown

  Scenario: Close button closes sleep timer modal
    Given the sleep timer modal is open
    When I click the close button
    Then the modal should close

  Scenario: Escape key closes sleep timer modal
    Given the sleep timer modal is open
    When I press Escape key
    Then the modal should close

  Scenario: Cancel button closes modal without starting timer
    Given the sleep timer modal is open
    When I click the Cancel button in modal footer
    Then the modal should close
    And no sleep timer should be active

  Scenario: Active sleep timer shows indicator on button
    Given a sleep timer is active
    When I view the player controls
    Then the sleep timer button should show an active indicator
    And the remaining minutes should be displayed on the button

  Scenario: Sleep timer modal has proper title and description
    Given the sleep timer modal is open
    Then the modal title should be Sleep Timer
    And a description should be visible
