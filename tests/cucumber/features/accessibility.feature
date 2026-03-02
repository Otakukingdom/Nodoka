@ui @accessibility
Feature: Accessibility Compliance
  As a user with disabilities
  I want the application to be accessible
  So that I can use it effectively with assistive technologies

  # ==================== WCAG Compliance ====================

  @critical
  Scenario: Application passes accessibility audit
    Given the application is running
    When I run an accessibility audit
    Then no critical accessibility violations should be found

  Scenario: All buttons have accessible labels
    Given the application is running
    Then all buttons should have accessible labels

  Scenario: All images have alt text
    Given the application is running
    Then all images should have alt text

  Scenario: Color contrast meets WCAG standards
    Given the application is running
    When I check color contrast accessibility
    Then all text should have sufficient color contrast

  # ==================== Focus Management ====================

  Scenario: Focus indicators are visible
    Given the application is running
    When I check for focus visibility
    Then focus indicators should be visible on all elements

  Scenario: Focus is trapped in modal
    Given the application is running
    And a modal is open for accessibility testing
    When I tab through all focusable elements
    Then focus should be trapped within the modal

  Scenario: Tab navigation works correctly
    Given the application is running
    When I navigate with keyboard only
    Then focus indicators should be visible on all elements

  # ==================== Modal Focus Trap UI Testing ====================

  Scenario: Tab cycles through modal elements only
    Given the settings modal is open
    When I press Tab 10 times
    Then focus should remain within the modal
    And focus should cycle through all focusable elements

  Scenario: Shift+Tab cycles backward in modal
    Given the settings modal is open
    When I press Shift+Tab 5 times
    Then focus should remain within the modal
    And focus should move backward through elements

  Scenario: Focus returns to trigger on modal close
    Given the application is running
    And the settings button is focused
    When I press Enter to open settings
    And I press Escape to close
    Then focus should return to the settings button

  Scenario: First focusable element gets focus on modal open
    Given the application is running
    When I open the settings modal
    Then the first focusable element should have focus

  Scenario: Screen reader announces modal title
    Given screen reader mode is enabled
    When the settings modal opens
    Then the modal title should be announced

  # ==================== Screen Reader Support ====================

  Scenario: State changes are announced
    Given the application is running
    And screen reader mode is enabled
    When I navigate with keyboard only
    Then a screen reader announcement should be made for state changes

  Scenario: Form inputs have labels
    Given the application is running
    Then form inputs should have associated labels

  Scenario: Interactive elements have appropriate roles
    Given the application is running
    Then interactive elements should have appropriate roles

  # ==================== Document Structure ====================

  Scenario: Valid heading structure
    Given the application is running
    Then the page should have a valid heading structure

  Scenario: Skip link is available
    Given the application is running
    Then the page should have a skip link

  # ==================== ARIA Usage ====================

  Scenario: aria-expanded is used correctly
    Given the application is running
    Then aria-expanded should be properly used

  Scenario: aria-hidden does not hide focusable elements
    Given the application is running
    Then aria-hidden should not hide focusable elements

  # ==================== Touch Targets ====================

  Scenario: Touch targets are large enough
    Given the application is running
    Then touch targets should be at least 44x44 pixels
