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

  # ==================== UI Performance Scenarios ====================

  Scenario: Cached cover loads instantly on revisit
    Given the application has an audiobook with cached cover art
    When I view the audiobook again
    Then the cover art should load from cache
    And the cover should appear without delay

  Scenario: Cache indicator shows during load
    Given the application has an audiobook with cover art
    When I view the audiobook for the first time
    Then a loading indicator may appear
    And the cover art should be displayed

  Scenario: Clear cache removes all cached covers
    Given the application has cached cover art for audiobooks
    When I clear the cover cache
    Then no covers should be cached
    And covers should reload on next view

  # ==================== UI Verification Scenarios ====================

  Scenario: Cover image loads and displays in img element
    Given the application has an audiobook with cover art
    When I view the audiobook
    Then the cover img element should be visible
    And the img src should point to the cover URL

  Scenario: Loading spinner appears while cover loads
    Given the application has an audiobook with cover art
    When I view the audiobook for the first time
    Then a loading spinner may appear before cover shows
    And the spinner should disappear when cover loads

  Scenario: Cached cover loads instantly without spinner
    Given the application has an audiobook with cached cover art
    When I view the audiobook again
    Then no loading spinner should appear
    And the cover should be visible immediately

  Scenario: Cover refresh shows updated image in UI
    Given the application has an audiobook with cached cover art
    When the cover art file is modified
    And I rescan the directory
    Then the updated cover should be visible in details panel
    And the library card should show the new cover

  Scenario: Cover error shows fallback placeholder
    Given the application has an audiobook with missing cover
    When I view the audiobook
    Then a placeholder image should be displayed
    And the placeholder should indicate no cover available
