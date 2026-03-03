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

  # ==================== Notification Stacking UI ====================

  Scenario: Multiple notifications stack vertically
    Given the application is running
    When I trigger 3 notifications in sequence
    Then 3 notifications should be visible
    And they should be stacked vertically

  Scenario: Newer notifications appear on top
    Given the application is running
    And 2 notifications are showing
    When I trigger another notification
    Then the new notification should appear on top

  Scenario: Dismiss all notifications button
    Given the application is running
    And 3 notifications are showing
    When I click dismiss all notifications
    Then all notifications should disappear

  Scenario: Notification shows correct icon for type
    Given the application is running
    When an error notification appears
    Then the notification should have an error icon

  Scenario: Success notification shows success icon
    Given the application is running
    When a success notification appears
    Then the notification should have a success icon

  Scenario: Info notification shows info icon
    Given the application is running
    When an info notification appears
    Then the notification should have an info icon

  Scenario: Notification shows progress indicator
    Given a notification is visible with progress
    Then a progress indicator should be shown on the notification

  # ==================== Timing and Auto-Dismiss UI Tests ====================

  Scenario: Notification auto-dismisses after configured duration
    Given a notification is visible
    When I wait 5 seconds
    Then the notification should disappear automatically

  Scenario: Notification dismiss countdown is visible
    Given a notification is visible with countdown
    Then the countdown timer should be displayed
    And the countdown should decrease over time

  Scenario: Hover pauses auto-dismiss countdown
    Given a notification is visible with countdown
    When I hover over the notification
    Then the countdown should pause
    When I move the mouse away
    Then the countdown should resume

  Scenario: Multiple notifications dismiss in correct order
    Given 3 notifications are showing
    When I wait for notifications to auto-dismiss
    Then notifications should dismiss in order of appearance

  Scenario: Swipe to dismiss notification on touch devices
    Given a notification is visible
    When I swipe the notification away
    Then the notification should disappear immediately
    And no dismiss animation should play
