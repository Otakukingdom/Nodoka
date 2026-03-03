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

  # ==================== UI Verification Scenarios ====================

  Scenario: Changed file indicator appears in UI
    Given the application has an audiobook with checksummed files
    When a file is modified externally
    And I rescan the directory
    Then the file should be flagged as changed
    And a visual change indicator should be visible on the file

  Scenario: Checksum verification progress indicator
    Given the library has an audiobook with large audio files
    When I scan the directory
    Then a scanning progress indicator should be visible
    And the progress indicator should show percentage complete

  Scenario: File integrity status displayed in details panel
    Given the application has an audiobook with checksummed files
    When I select the audiobook
    Then the file integrity status should be displayed in the details panel
    And each file should show a verification status icon

  Scenario: Checksum status tooltip on hover
    Given the application has an audiobook with checksummed files
    When I hover over a file verification status icon
    Then a tooltip should show the checksum verification details

  Scenario: Checksum mismatch shows warning notification
    Given the application has an audiobook with checksummed files
    When a file is modified externally
    And I rescan the directory
    Then a warning notification should appear
    And the notification should indicate file integrity issues
