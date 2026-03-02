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
    And I set user preference "language" to "en"
    When I get user preference "language"
    Then the value should be "en"

  Scenario: Getting all user preferences
    Given the application has a database
    And I set user preference "language" to "en"
    And I set user preference "region" to "US"
    When I get all user preferences
    Then I should receive 2 preferences

  Scenario: Deleting a user preference
    Given the application has a database
    And I set user preference "language" to "en"
    When I delete user preference "language"
    Then the preference should not exist

  Scenario: Updating existing preference
    Given the application has a database
    And I set user preference "language" to "en"
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
