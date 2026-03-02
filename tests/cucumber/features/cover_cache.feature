@ui
Feature: Cover Art Cache
  As an audiobook listener
  I want cover art to be cached efficiently
  So that the application performs well

  @critical
  Scenario: Cover art is cached on first load
    Given the application has an audiobook with cover art
    When I view the audiobook
    Then the cover art should be displayed
    And the cover art should be cached

  Scenario: Cached cover art loads faster on subsequent views
    Given the application has an audiobook with cached cover art
    When I view the audiobook again
    Then the cover art should load from cache
    And the cover art should be displayed

  Scenario: Cache is invalidated when cover changes
    Given the application has an audiobook with cached cover art
    When the cover art file is modified
    And I rescan the directory
    Then the cache should be updated
    And the new cover art should be displayed

  Scenario: Multiple audiobooks cache independently
    Given the application has 3 audiobooks with cover art
    When I view each audiobook
    Then all cover art should be cached
    And each audiobook should display its own cover

  Scenario: Cache persists across application restarts
    Given the application has an audiobook with cached cover art
    When I restart the application
    And I view the audiobook
    Then the cover art should load from cache
