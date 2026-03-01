Feature: Directory Scanning
  As an audiobook listener
  I want to scan directories for audiobooks
  So that my library is automatically populated

  Scenario: Adding a directory stores it in database
    Given the application has an empty database
    When I add a directory to the library
    Then the directory should be stored in the database
    And the directory count should be one

  Scenario: Directory can be removed from library
    Given the application has a directory
    When I remove the directory
    Then the directory count should be zero

  Scenario: Multiple directories can be added
    Given the application has an empty database
    When I add a directory to the library
    And I add a directory to the library
    Then the directory count should be two

  Scenario: Directory metadata is tracked
    Given the application has a directory
    When I scan the directory
    Then the directory last scanned timestamp should be updated

  Scenario: Directory path is preserved
    Given the application has a directory
    When I retrieve the directory
    Then the directory path should match the original

  Scenario: Directory removal cleans up audiobooks
    Given the application has a directory
    And the application has an audiobook
    When I remove the directory
    Then the directory count should be zero
