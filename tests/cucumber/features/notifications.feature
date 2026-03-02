@ui @notifications
Feature: Notification System
  As a user
  I want to receive feedback about application operations
  So that I know when actions succeed or fail

  # Note: Basic success/error notification scenarios are in ui_components.feature

  # ==================== Notification Types ====================

  Scenario: Info notification displays
    Given the application is running
    When I trigger an info notification
    Then an info notification should appear

  # ==================== Auto-Dismiss ====================

  Scenario: Notification dismisses automatically after duration
    Given a notification is visible
    When I wait for the notification duration to expire
    Then the notification should disappear automatically

  # ==================== Manual Dismiss ====================

  Scenario: Notification can be dismissed manually
    Given a notification is visible
    When I click the notification dismiss button
    Then the notification should not be visible

  Scenario: Notification can be dismissed by clicking
    Given a notification is visible
    When I click on the notification
    Then the notification should handle the click action

  # ==================== Multiple Notifications ====================

  Scenario: Multiple notifications stack properly
    Given multiple notifications are showing
    Then multiple notifications should stack
    And newer notifications should appear above older ones

  # ==================== Notification Content ====================

  Scenario: Notification displays message content
    Given a notification is visible
    Then the notification should have an icon
    And the notification should be dismissible
