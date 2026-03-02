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
