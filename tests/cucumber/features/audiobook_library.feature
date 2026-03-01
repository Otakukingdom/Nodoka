Feature: Audiobook Library Management
  As an audiobook listener
  I want to manage my audiobook library
  So that I can organize and access my audiobook collection

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
