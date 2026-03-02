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

  # UI-specific scenarios

  Scenario: Playable files show play icon
    Given the library has an audiobook with valid audio files
    When I view the files list
    Then each playable file should have a play icon

  Scenario: Format badge shows file type
    Given the library has an audiobook with mixed audio formats
    When I view the files list
    Then each file should display its format as a badge

  Scenario: Duration is formatted as HH:MM:SS
    Given the library has an audiobook with valid audio files
    When I view the files list
    Then the duration should be in HH:MM:SS or MM:SS format

  Scenario: Corrupted file shows warning icon
    Given the library has an audiobook with corrupted audio
    When I view the files list
    Then the corrupted file should have a warning icon

  Scenario: Unsupported format shows error indicator
    Given the library has an audiobook with unsupported file formats
    When I view the files list
    Then the unsupported files should have an error indicator

  Scenario: File size is displayed
    Given the library has an audiobook with valid audio files
    When I view the files list
    Then each file should display its file size

  Scenario: Bitrate information is shown
    Given the library has an audiobook with MP3 files
    When I view the file details
    Then the bitrate should be displayed

  Scenario: Sample rate is displayed for high-quality files
    Given the library has an audiobook with FLAC files
    When I view the file details
    Then the sample rate should be displayed

  # ==================== Duration Calculation ====================

  @critical
  Scenario: Duration is calculated correctly for MP3 files
    Given the library has an audiobook with MP3 files
    When I view the files list
    Then each file should display its calculated duration
    And the duration should match the actual audio length

  Scenario: Total audiobook duration is sum of file durations
    Given the library has an audiobook with 3 files
    When I view the audiobook details
    Then the total duration should be displayed
    And the total should equal the sum of all file durations

  Scenario: Duration updates when file is replaced
    Given the library has an audiobook with a file
    When the file is replaced with a longer version
    And I rescan the directory
    Then the duration should be updated
    And the audiobook total duration should reflect the change

  Scenario: Duration is preserved across sessions
    Given the library has an audiobook with known duration
    When I restart the application
    Then the file durations should be preserved
    And the total audiobook duration should be correct

  Scenario: Duration displays correctly for very long files
    Given the library has an audiobook with a file over 24 hours
    When I view the files list
    Then the duration should display in HH:MM:SS format
    And hours should be correctly calculated