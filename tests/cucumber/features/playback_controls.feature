Feature: Playback Controls
  As an audiobook listener
  I want to control audio playback
  So that I can listen to my audiobooks with precise control

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
