@ui
Feature: Audiobook Library Management
  As an audiobook listener
  I want to manage my audiobook library
  So that I can organize and access my audiobook collection

  @critical
  Scenario: Adding a directory to the library
    Given the application has an empty database
    When I add a directory to the library
    Then the directory should be stored in the database
    And the directory count should be one

  Scenario: Listing all audiobooks in an empty library
    Given the application has an empty database
    When I request all audiobooks
    Then I should receive an empty list

  Scenario: Adding an audiobook to the library
    Given the application has an empty database
    When I add an audiobook
    Then the audiobook should be stored in the database
    And the audiobook count should be one

  Scenario: Getting audiobook by ID
    Given the application has an audiobook
    When I request the audiobook by its ID
    Then the completeness should be zero

  Scenario: Getting files for an audiobook
    Given the application has an audiobook
    And the audiobook has files
    When I request the files for the audiobook
    Then I should receive three files
    And the files should be ordered by position

  Scenario: Removing a directory
    Given the application has a directory
    When I remove the directory
    Then the directory count should be zero

  Scenario: Adding multiple audiobooks
    Given the application has an empty database
    When I add an audiobook
    And I add an audiobook
    Then the audiobook count should be two

  # Extended scenarios for deletion and update operations

  Scenario: Deleting an audiobook removes it from database
    Given the application has an audiobook
    When I delete the audiobook
    Then the audiobook count should be zero

  Scenario: Deleting audiobook cascades to files
    Given the application has an audiobook
    And the audiobook has files
    When I delete the audiobook
    Then the audiobook count should be zero
    And the file count for the audiobook should be zero

  Scenario: Deleting audiobook cascades to bookmarks
    Given the application has an audiobook
    And the audiobook has bookmarks
    When I delete the audiobook
    Then the bookmark count for the audiobook should be zero

  Scenario: Updating audiobook author
    Given the application has an audiobook
    When I set the audiobook author to "New Author"
    Then the audiobook author should be "New Author"

  Scenario: Clearing audiobook author
    Given the application has an audiobook with author
    When I clear the audiobook author
    Then the audiobook author should be None

  Scenario: Updating audiobook total duration
    Given the application has an audiobook
    When I set the audiobook total duration to 3600000 ms
    Then the audiobook total duration should be 3600000

  Scenario: Marking audiobook as complete
    Given the application has an audiobook
    And the audiobook has files
    When I mark the audiobook as complete
    Then the audiobook completeness should be 100
    And all files should be 100 percent complete

  Scenario: Resetting audiobook progress
    Given the application has an audiobook
    And the audiobook has files with progress
    When I reset the audiobook progress
    Then the audiobook completeness should be zero
    And all files should have zero progress

  Scenario: Getting audiobook by path
    Given the application has an audiobook
    When I request the audiobook by path
    Then the audiobook should be found

  Scenario: Getting audiobooks by directory
    Given the application has a directory
    And the application has an audiobook in that directory
    When I request audiobooks for the directory
    Then I should receive one audiobook
