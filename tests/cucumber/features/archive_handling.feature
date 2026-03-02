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

  # ==================== Extended Archive UI Interaction Scenarios ====================

  Scenario: Password prompt UI for protected archives
    Given the library has a password protected archive
    When I select the audiobook
    Then a password prompt modal should appear
    And the password input field should be focused
    And a submit button should be visible

  Scenario: Enter password and access archive
    Given the library has a password protected archive
    And I select the audiobook
    When I enter the correct password
    Then the archive should be unlocked
    And the files should be listed

  Scenario: Wrong password shows error
    Given the library has a password protected archive
    And I select the audiobook
    When I enter an incorrect password
    Then an error message should be displayed
    And the password prompt should remain visible

  Scenario: Cancel password prompt
    Given the library has a password protected archive
    And I select the audiobook
    When I click cancel on the password prompt
    Then the password prompt should close
    And the audiobook should not be selected

  Scenario: Nested archive folder expansion
    Given the library has a nested archive
    When I view the files list
    And I click on a folder
    Then the folder contents should be displayed
    And nested files should be accessible

  Scenario: Archive extraction progress indicator
    Given the library has a large ZIP archive audiobook
    When I select the audiobook
    Then an extraction progress bar should be visible
    And the progress percentage should be displayed
    And the progress should update during extraction

  Scenario: Archive type badge for ZIP
    Given the library has a ZIP archive audiobook
    When I view the audiobook card
    Then a ZIP badge should be visible

  Scenario: Archive type badge for 7z
    Given the library has a 7z archive audiobook
    When I view the audiobook card
    Then a 7z badge should be visible

  Scenario: Archive file size display
    Given the library has a ZIP archive audiobook
    When I view the audiobook details
    Then the archive file size should be displayed
    And the size should be in MB or GB format

  Scenario: Archive icon on library card
    Given the library has a ZIP archive audiobook
    When I view the library
    Then the audiobook card should have an archive icon
    And hovering the icon should show archive type tooltip

  Scenario: Files from archive show archive indicator
    Given the library has a ZIP archive audiobook
    When I select the audiobook
    And I view the files list
    Then each file should show an archive source indicator

  Scenario: Corrupted archive shows detailed error
    Given the library has a corrupted archive
    When I try to select the audiobook
    Then an error message should be displayed
    And the error should mention archive corruption
    And a suggestion to re-download should be shown
