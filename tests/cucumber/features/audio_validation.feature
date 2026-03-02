@ui
Feature: Audio File Validation
  As an audiobook listener
  I want audio files to be validated
  So that I know which files can be played

  @critical
  Scenario: Valid audio file is recognized
    Given the library has an audiobook with valid audio files
    When I view the files list
    Then the files should be marked as playable

  Scenario: Unsupported file format is indicated
    Given the library has an audiobook with unsupported file formats
    When I view the files list
    Then the unsupported files should be marked accordingly

  Scenario: Corrupted audio file shows warning
    Given the library has an audiobook with corrupted audio
    When I try to play the corrupted file
    Then a warning should be displayed
    And playback should not start

  Scenario: MP3 files are supported
    Given the library has an audiobook with MP3 files
    When I select a file
    Then the file should be playable

  Scenario: M4B files are supported
    Given the library has an audiobook with M4B files
    When I select a file
    Then the file should be playable

  Scenario: FLAC files are supported
    Given the library has an audiobook with FLAC files
    When I select a file
    Then the file should be playable

  Scenario: OGG files are supported
    Given the library has an audiobook with OGG files
    When I select a file
    Then the file should be playable

  Scenario: WAV files are supported
    Given the library has an audiobook with WAV files
    When I select a file
    Then the file should be playable

  Scenario: File duration is calculated
    Given the library has an audiobook with valid audio files
    When I view the files list
    Then each file should display its duration
