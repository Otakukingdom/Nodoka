@ui
Feature: Bookmark Management
  As an audiobook listener
  I want to create and manage bookmarks
  So that I can mark important positions in my audiobooks

  @critical
  Scenario: Creating a bookmark
    Given the application has an audiobook
    And the audiobook has a file
    When I create a bookmark
    Then the bookmark should be stored in the database
    And the bookmark count should be one

  Scenario: Listing bookmarks for an audiobook
    Given the application has an audiobook
    And the audiobook has bookmarks
    When I request the bookmarks for the audiobook
    Then I should receive three bookmarks

  Scenario: Deleting a bookmark
    Given the application has an audiobook
    And the audiobook has bookmarks
    When I delete a bookmark
    Then the bookmark count should be two

  Scenario: Bookmark with optional note
    Given the application has an audiobook
    And the audiobook has a file
    When I create a bookmark with note
    Then the bookmark should be stored in the database
    And the bookmark should have note

  # Extended scenarios for bookmark updates

  Scenario: Updating bookmark label
    Given the application has an audiobook
    And the audiobook has a bookmark
    When I update the bookmark label to "Updated Label"
    Then the bookmark label should be "Updated Label"

  Scenario: Updating bookmark note
    Given the application has an audiobook
    And the audiobook has a bookmark
    When I update the bookmark note to "Updated note"
    Then the bookmark note should be "Updated note"

  Scenario: Removing bookmark note
    Given the application has an audiobook
    And the audiobook has a bookmark with note
    When I remove the bookmark note
    Then the bookmark note should be None

  Scenario: Bookmarks are ordered by file position then time
    Given the application has an audiobook
    And the audiobook has multiple files
    And the audiobook has bookmarks in different files
    When I request the bookmarks for the audiobook
    Then the bookmarks should be ordered by file position
    And bookmarks in the same file should be ordered by time
