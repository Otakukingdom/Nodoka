@ui
Feature: Archive Handling
  As an audiobook listener
  I want to play audiobooks from archives
  So that I can listen to compressed collections

  @critical
  Scenario: ZIP archive is extracted and played
    Given the library has a ZIP archive audiobook
    When I select the audiobook
    Then the files should be listed from the archive
    And I can play files from the archive

  Scenario: RAR archive is supported
    Given the library has a RAR archive audiobook
    When I select the audiobook
    Then the files should be listed from the archive

  Scenario: 7z archive is supported
    Given the library has a 7z archive audiobook
    When I select the audiobook
    Then the files should be listed from the archive

  Scenario: Nested archives are handled
    Given the library has a nested archive
    When I select the audiobook
    Then the innermost files should be accessible

  Scenario: Archive with multiple audiobooks
    Given the library has a ZIP archive with multiple audiobooks
    When I scan the archive
    Then each audiobook should be listed separately

  Scenario: Corrupted archive shows error
    Given the library has a corrupted archive
    When I try to select the audiobook
    Then an error message should be displayed

  Scenario: Archive extraction preserves file order
    Given the library has a ZIP archive audiobook
    When I view the files list
    Then the files should be ordered by position

  Scenario: Password protected archive prompts for password
    Given the library has a password protected archive
    When I select the audiobook
    Then a password prompt should appear

  # UI-specific scenarios

  Scenario: Archive shows zip icon indicator
    Given the library has a ZIP archive audiobook
    When I view the audiobook in the library
    Then an archive icon should be visible on the audiobook card

  Scenario: Archive extraction shows progress
    Given the library has a large ZIP archive audiobook
    When I select the audiobook
    Then an extraction progress indicator should be displayed

  Scenario: Extracted files count is displayed
    Given the library has a ZIP archive audiobook
    When I select the audiobook
    Then the total file count should be shown in the details panel

  Scenario: Archive file size is shown
    Given the library has a ZIP archive audiobook
    When I view the audiobook details
    Then the archive file size should be displayed

  Scenario: Nested archive shows folder structure
    Given the library has a nested archive
    When I view the files list
    Then the folder hierarchy should be visible

  Scenario: Archive indicator in file list
    Given the library has a ZIP archive audiobook
    When I view the files list
    Then each file should show it is from an archive

  Scenario: Archive type badge is displayed
    Given the library has a RAR archive audiobook
    When I view the audiobook card
    Then a RAR badge should be visible
