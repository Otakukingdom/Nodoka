@ui
Feature: File Checksum Verification
  As an audiobook listener
  I want file integrity to be verified
  So that I know files have not been corrupted

  @critical
  Scenario: Checksum is calculated for new files
    Given the application has an audiobook
    When I add a new file to the audiobook
    Then a checksum should be calculated
    And the checksum should be stored

  Scenario: Checksum detects file changes
    Given the application has an audiobook with checksummed files
    When a file is modified externally
    And I rescan the directory
    Then the file should be flagged as changed

  Scenario: Checksum verifies file integrity on load
    Given the application has an audiobook with checksummed files
    When I load the audiobook
    Then the files should be verified against their checksums

  Scenario: Multiple checksum algorithms are supported
    Given the application has an audiobook
    When files are scanned
    Then checksums should be calculated using SHA-256

  Scenario: Checksum is recalculated on rescan
    Given the application has an audiobook with checksummed files
    When I force a full rescan
    Then all checksums should be recalculated
    And new checksums should be stored

  Scenario: Large files checksum calculation
    Given the library has an audiobook with large audio files
    When I scan the directory
    Then checksums should be calculated without blocking the UI
