@ui
Feature: Directory Management UX
  As an audiobook listener
  I want an intuitive interface for managing my audiobook directories
  So that I can quickly add and organize my audiobook library

  Background:
    Given the application is running

  # ==================== First-Time User Experience ====================

  @critical @ux @first-time
  Scenario: First-time user sees empty state with prominent Add Folder button
    Given I am a first-time user with no directories
    When I open the application
    Then I should see an empty state message
    And I should see an "Add Folder" button as the primary action
    And the "Add Folder" button should be prominently displayed
    And I should see keyboard shortcut hints for adding a folder

  @ux @first-time
  Scenario: First-time user can add folder within 30 seconds
    Given I am a first-time user with no directories
    When I open the application
    And I click the "Add Folder" button
    Then a folder picker dialog should open
    When I select a folder containing audiobooks
    Then I should see scanning progress indicator within 100ms
    And I should see audiobooks appear in the library
    And the total time should be under 30 seconds

  @ux @empty-state
  Scenario: Empty state shows helpful guidance
    Given I am a first-time user with no directories
    When I open the application
    Then I should see guidance text explaining how to add audiobooks
    And I should see drag and drop hint
    And the primary action should be clear and obvious

  # ==================== Keyboard Shortcuts ====================

  @keyboard @ux
  Scenario: Add directory via keyboard shortcut
    Given I am viewing the library
    When I press Ctrl+D keys
    Then a folder picker dialog should open

  @keyboard @ux
  Scenario: Rescan directory via keyboard shortcut
    Given I have at least one directory in my library
    When I press Ctrl+R keys
    Then the directory should be rescanned
    And I should see scanning progress indicator

  @keyboard @ux
  Scenario: Cancel operation with Escape key
    Given a folder picker dialog is open
    When I press Escape key
    Then the folder picker dialog should close
    And no directory should be added

  # ==================== Drag and Drop ====================

  @ux @drag-drop
  Scenario: Add directory via drag and drop
    Given I am viewing the library
    When I drag a folder onto the library panel
    Then the folder should be added as a directory
    And I should see scanning progress indicator

  @ux @drag-drop
  Scenario: Drag and drop shows visual feedback
    Given I am viewing the library
    When I start dragging a folder over the library panel
    Then I should see a drop target highlight

  # ==================== Error Handling and Recovery ====================

  @ux @error
  Scenario: Directory not found shows recovery options
    Given I have a directory that has been moved or deleted
    When I open the application
    Then I should see an error banner for the missing directory
    And the error should explain the problem in plain language
    And I should see a "Locate" button to find the folder
    And I should see a "Remove" button to remove the directory

  @ux @error
  Scenario: Relocate missing directory
    Given I have a directory that has been moved
    And I see an error banner for the missing directory
    When I click the "Locate" button
    Then a folder picker should open
    When I select the new location of the folder
    Then the directory path should be updated
    And the error banner should disappear
    And the audiobooks should be accessible again

  @ux @error
  Scenario: Remove directory with error
    Given I have a directory that has an error
    And I see an error banner for the directory
    When I click the "Remove" button
    Then the directory should be removed from the library
    And the error banner should disappear

  @ux @error
  Scenario: Permission denied error shows helpful message
    Given I have a directory with permission issues
    When I open the application
    Then I should see an error banner explaining permission denied
    And I should see a "Remove" button

  # ==================== Scanning Progress ====================

  @ux @scanning
  Scenario: Long scan shows progress indicator
    Given I am adding a large directory
    When the scan takes more than 2 seconds
    Then I should see elapsed time in the progress indicator
    And I should see the current path being scanned
    And I should see the count of audiobooks found

  @ux @scanning
  Scenario: Scan progress shows audiobook count
    Given I am scanning a directory
    When audiobooks are found during scanning
    Then I should see the count of audiobooks found
    And the count should update in real-time

  @ux @scanning
  Scenario: Cancel ongoing scan
    Given a directory scan is in progress
    When I click the cancel button on the progress indicator
    Then the scan should be cancelled
    And the progress indicator should disappear
    And any audiobooks already found should remain in the library

  # ==================== Loading States ====================

  @ux @loading
  Scenario: Library shows skeleton during initial load
    Given I am a first-time user with no directories
    And the library is loading
    Then I should see skeleton loading placeholders
    And the skeletons should resemble audiobook cards

  @ux @loading
  Scenario: Add folder button shows loading state
    Given I am adding a directory
    When the directory is being scanned
    Then the "Add Folder" button should show a loading spinner
    And the button should be disabled during scanning

  # ==================== Accessibility ====================

  @accessibility @ux
  Scenario: Keyboard navigation through library
    Given I have audiobooks in my library
    When I press Tab key
    Then focus should move to the next interactive element
    And focus indicators should be clearly visible

  @accessibility @ux
  Scenario: Empty state is accessible
    Given I am a first-time user with no directories
    When I open the application
    Then the "Add Folder" button should be keyboard focusable
    And the button should have an accessible name
    And pressing Enter should activate the button

  @accessibility @ux
  Scenario: Error banners are announced to screen readers
    Given I have a directory that has an error
    When I open the application
    Then the error banner should have role="alert"
    And the error message should be announced

  # ==================== Visual Feedback ====================

  @ux @visual
  Scenario: Button hover states are visible
    Given I am viewing the empty state
    When I hover over the "Add Folder" button
    Then I should see a visible hover effect
    And the button should respond to interaction

  @ux @visual
  Scenario: Scanning indicator animates
    Given a directory scan is in progress
    When I view the progress indicator
    Then I should see an animated scanning icon
    And the animation should indicate activity

  # ==================== Returning User Experience ====================

  @ux @returning
  Scenario: Returning user can quickly add another directory
    Given I have existing audiobooks in my library
    When I want to add another folder
    Then I should see an add folder button within 3 seconds
    And the button should be accessible from the main view

  @ux @returning
  Scenario: Directory list shows relevant information
    Given I have multiple directories in my library
    When I view the directory list
    Then I should see the path for each directory
    And I should see the audiobook count for each directory
    And I should see the last scanned timestamp
