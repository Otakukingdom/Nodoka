@ui
Feature: User Preferences Management
  As an audiobook listener
  I want my preferences to be stored separately from system metadata
  So that my settings are organized and easily retrievable

  @critical
  Scenario: Setting a user preference
    Given the application has a database
    When I set user preference "language" to "en"
    Then the preference should be stored

  Scenario: Getting a user preference
    Given the application has a database
    And the application has user preference "language" set to "en"
    When I get user preference "language"
    Then the value should be "en"

  Scenario: Getting all user preferences
    Given the application has a database
    And the application has user preference "language" set to "en"
    And the application has user preference "region" set to "US"
    When I get all user preferences
    Then I should receive 2 preferences

  Scenario: Deleting a user preference
    Given the application has a database
    And the application has user preference "language" set to "en"
    When I delete user preference "language"
    Then the preference should not exist

  Scenario: Updating existing preference
    Given the application has a database
    And the application has user preference "language" set to "en"
    When I set user preference "language" to "es"
    Then the preference value should be "es"

  Scenario: Preference rejects NUL bytes in key
    Given the application has a database
    When I try to set user preference with NUL byte key
    Then the operation should fail

  Scenario: Preference rejects NUL bytes in value
    Given the application has a database
    When I try to set user preference with NUL byte value
    Then the operation should fail

  # ==================== UI Interaction Scenarios ====================

  Scenario: User preferences accessible via settings modal
    Given the application is running
    When I open the settings modal
    Then the preferences section should be visible
    And preference controls should be interactive

  Scenario: Language preference dropdown changes UI language
    Given the application is running
    And I open the settings modal
    When I select language "es" from the language dropdown
    Then the UI language should change to Spanish
    And the language preference should be saved

  Scenario: Preference changes persist after modal close
    Given the application is running
    And I open the settings modal
    When I toggle the dark mode preference
    And I close the settings modal
    And I reopen the settings modal
    Then the dark mode preference should still be enabled

  Scenario: Preference reset button clears custom preferences
    Given the application is running
    And I have modified several preferences
    When I open the settings modal
    And I click the reset to defaults button
    Then all preferences should return to default values
    And a confirmation notification should appear

  Scenario: Skip duration preference updates player controls
    Given the application is running
    And I open the settings modal
    When I set skip forward duration to 60 seconds
    Then the skip forward button should show 60 seconds
    And the preference should be saved

  Scenario: Volume preference persists across sessions
    Given the application is running
    And I set the volume to 75
    When I restart the application
    Then the volume should be restored to 75

  Scenario: Theme preference applies immediately
    Given the application is running
    When I change the theme preference to "dark"
    Then the dark theme should be applied immediately
    And the theme toggle should show dark mode active

  Scenario: Preference form validation shows errors
    Given the application is running
    And I open the settings modal
    When I enter an invalid value in a preference field
    Then a validation error should be displayed
    And the save button should be disabled
