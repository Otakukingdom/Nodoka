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
