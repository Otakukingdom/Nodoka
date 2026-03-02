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

  # ==================== Bookmark UI Interaction Scenarios ====================

  Scenario: Create bookmark via Ctrl+B keyboard shortcut
    Given the application has an audiobook
    And a file is playing
    When I press Ctrl+B keys
    Then a bookmark should be created at current position
    And the bookmark should appear in the bookmarks list

  Scenario: Create bookmark via context menu
    Given the application has an audiobook
    And a file is loaded
    When I right-click on the seek bar
    And I select add bookmark from context menu
    Then a bookmark should be created

  Scenario: Click bookmark to jump to position
    Given the application has an audiobook
    And the audiobook has bookmarks
    When I click on the first bookmark
    Then playback position should jump to bookmark position
    And the file containing bookmark should be selected

  Scenario: Edit bookmark label inline
    Given the application has an audiobook
    And the audiobook has a bookmark
    When I double-click on the bookmark label
    And I type "Important chapter"
    And I press Enter
    Then the bookmark label should be "Important chapter"

  Scenario: Edit bookmark note inline
    Given the application has an audiobook
    And the audiobook has a bookmark
    When I hover over the bookmark
    And I click the edit note button
    And I type "This is where the hero meets the villain"
    And I press Enter
    Then the bookmark note should be updated

  Scenario: Delete bookmark via delete button
    Given the application has an audiobook
    And the audiobook has bookmarks
    When I hover over the first bookmark
    And I click the delete button
    Then the bookmark should be removed from the list
    And the bookmark count should decrease by one

  Scenario: Bookmark displays time in correct format
    Given the application has an audiobook
    And a bookmark exists at 5 minutes 30 seconds
    When I view the bookmarks list
    Then the bookmark time should display as "5:30"

  Scenario: Bookmark displays label and time
    Given the application has an audiobook
    And a bookmark with label "Chapter 5" at 10 minutes
    When I view the bookmarks section
    Then the label "Chapter 5" should be visible
    And the time "10:00" should be visible

  Scenario: Empty bookmarks shows placeholder message
    Given the application has an audiobook
    And the audiobook has no bookmarks
    When I view the bookmarks section
    Then a no bookmarks message should be displayed
    And a hint about creating bookmarks should be shown

  Scenario: Bookmarks section is scrollable with many bookmarks
    Given the application has an audiobook
    And the audiobook has 20 bookmarks
    When I view the bookmarks section
    Then the bookmarks list should be scrollable
    And all bookmarks should be accessible via scrolling

  Scenario: Bookmark icon indicates bookmarked position
    Given the application has an audiobook
    And the audiobook has bookmarks
    When I view the seek bar
    Then bookmark indicators should be visible at bookmark positions

  Scenario: Create bookmark stores current file position
    Given the application has an audiobook
    And a file is playing at position 3 minutes 45 seconds
    When I create a bookmark
    Then the bookmark should store position 3:45
    And the bookmark should be associated with the current file
