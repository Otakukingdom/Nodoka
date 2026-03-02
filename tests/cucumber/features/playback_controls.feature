@ui
Feature: Playback Controls
  As an audiobook listener
  I want to control audio playback
  So that I can listen to my audiobooks with precise control

  @critical
  Scenario: Volume is clamped to maximum
    Given the application has a volume setting
    When I set the volume above maximum
    Then the volume should be clamped to maximum

  Scenario: Volume is clamped to minimum
    Given the application has a volume setting
    When I set the volume below minimum
    Then the volume should be clamped to minimum

  Scenario: Speed is clamped to maximum
    Given the application has a speed setting
    When I set the speed above maximum
    Then the speed should be clamped to maximum

  Scenario: Speed is clamped to minimum
    Given the application has a speed setting
    When I set the speed below minimum
    Then the speed should be clamped to minimum

  Scenario: Valid volume is accepted
    Given the application has a volume setting
    When I set the volume to valid value
    Then the volume should be valid

  Scenario: Valid speed is accepted
    Given the application has a speed setting
    When I set the speed to valid value
    Then the speed should be valid

  # Extended scenarios for playback state management

  Scenario: Initial playback state is NothingSpecial
    Given the application has a player
    When I check the playback state
    Then the playback state should be NothingSpecial

  Scenario: Playback state changes to Playing
    Given the application has a player
    And the player has loaded media
    When I start playback
    Then the playback state should be Playing

  Scenario: Playback state changes to Paused
    Given the application has a player
    And the player is playing
    When I pause playback
    Then the playback state should be Paused

  Scenario: Playback state changes to Stopped
    Given the application has a player
    And the player is playing
    When I stop playback
    Then the playback state should be Stopped

  Scenario: Playback state changes to Ended
    Given the application has a player
    And the player is playing
    When playback reaches the end
    Then the playback state should be Ended

  Scenario: Seek position is clamped to valid range
    Given the application has a player
    And the player has loaded media with duration 60000 ms
    When I seek to position 70000 ms
    Then the position should be clamped to duration
