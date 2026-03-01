Feature: Bookmark Management
  As an audiobook listener
  I want to create and manage bookmarks
  So that I can mark important positions in my audiobooks

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
