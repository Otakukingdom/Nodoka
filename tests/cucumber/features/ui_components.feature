@ui
Feature: UI Component Behavior
  As an audiobook listener
  I want a responsive and accessible user interface
  So that I can efficiently interact with the application

  # ==================== Library Loading ====================

  @critical
  Scenario: Application loads and displays library
    Given the application has an audiobook
    When I request all audiobooks
    Then the audiobook count should be one

  Scenario: Library displays multiple audiobooks
    Given the application has 3 audiobooks
    When I request all audiobooks
    Then the audiobook count should be 3

  Scenario: Empty library shows placeholder
    Given the application has no audiobooks
    When I view the library
    Then an empty state message should be displayed

  # ==================== Audiobook Selection ====================

  Scenario: Selecting an audiobook displays files
    Given the application has an audiobook
    And the audiobook has files
    When I request the files for the audiobook
    Then I should receive three files
    And the files should be ordered by position

  Scenario: Selected audiobook is highlighted
    Given the application has 2 audiobooks
    When I select the first audiobook
    Then the audiobook should be visually highlighted

  Scenario: Selecting audiobook shows details panel
    Given the application has an audiobook
    When I select an audiobook
    Then the details panel should be visible
    And the audiobook title should be displayed

  Scenario: Audiobook author is displayed when available
    Given the application has an audiobook with author "Jane Author"
    When I view the audiobook
    Then the author "Jane Author" should be displayed

  Scenario: Audiobook without author shows placeholder
    Given the application has an audiobook without author
    When I view the audiobook
    Then an unknown author placeholder should be displayed

  # ==================== Files List ====================

  Scenario: Files list shows all chapters
    Given the application has an audiobook
    And the audiobook has 5 files
    When I view the audiobook details
    Then 5 files should be listed

  Scenario: File progress is displayed
    Given the application has an audiobook
    And a file with 50% progress
    When I view the files list
    Then the file progress should show 50%

  Scenario: Missing file is indicated
    Given the application has an audiobook
    And a file that does not exist
    When I view the files list
    Then the file should be marked as missing

  Scenario: File duration is formatted correctly
    Given the application has an audiobook
    And a file with duration 3661000 ms
    When I view the files list
    Then the duration should display as "1:01:01"

  # ==================== Bookmarks Section ====================

  Scenario: Bookmarks can be displayed for selected audiobook
    Given the application has an audiobook
    And the audiobook has bookmarks
    When I request the bookmarks for the audiobook
    Then I should receive three bookmarks

  Scenario: Bookmark list shows label and time
    Given the application has an audiobook
    And a bookmark with label "Important scene" at 5 minutes
    When I view the bookmarks
    Then the label "Important scene" should be visible
    And the time "5:00" should be visible

  Scenario: Empty bookmarks shows placeholder
    Given the application has an audiobook
    And the audiobook has no bookmarks
    When I view the bookmarks section
    Then a no bookmarks message should be displayed

  Scenario: Bookmark can be created from current position
    Given the application has an audiobook
    And a file is playing at position 2 minutes
    When I create a bookmark with label "Chapter start"
    Then the bookmark should be saved
    And the bookmark should appear in the list

  # ==================== Player Controls ====================

  Scenario: Player controls respond to state changes
    Given the application has an audiobook
    And the audiobook has a file
    When I set the volume to valid value
    Then the volume should be valid

  Scenario: Speed controls respond to state changes
    Given the application has an audiobook
    And the audiobook has a file
    When I set the speed to valid value
    Then the speed should be valid

  Scenario: Play button toggles to pause when playing
    Given the application has an audiobook
    And a file is loaded
    When I click the play button
    Then the button should change to pause icon

  Scenario: Pause button toggles to play when paused
    Given the application has an audiobook
    And a file is playing
    When I click the pause button
    Then the button should change to play icon

  Scenario: Stop button resets playback
    Given the application has an audiobook
    And a file is playing
    When I click the stop button
    Then playback should stop
    And the position should reset to start

  Scenario: Volume slider updates volume
    Given the application has an audiobook
    And a file is loaded
    When I drag the volume slider to 75%
    Then the volume should be set to 75

  Scenario: Mute button toggles mute state
    Given the application has an audiobook
    And a file is playing with volume
    When I click the mute button
    Then the audio should be muted

  Scenario: Speed button cycles through speeds
    Given the application has an audiobook
    And a file is loaded
    When I click the speed button
    Then the speed should cycle to the next value

  Scenario: Seek bar shows current position
    Given the application has an audiobook
    And a file is playing at 30% progress
    When I view the seek bar
    Then the seek bar should be at 30%

  Scenario: Skip forward button advances position
    Given the application has an audiobook
    And a file is playing
    When I click the skip forward button
    Then the position should advance by the skip duration

  Scenario: Skip backward button rewinds position
    Given the application has an audiobook
    And a file is playing
    When I click the skip backward button
    Then the position should rewind by the skip duration

  Scenario: Sleep timer button opens modal
    Given the application is running
    When I click the sleep timer button
    Then the sleep timer modal should open

  # ==================== Top Bar ====================

  Scenario: Settings button opens settings modal
    Given the application is running
    When I click the settings button
    Then the settings modal should open

  Scenario: Theme toggle switches themes
    Given the application is in light theme
    When I toggle the theme
    Then the theme should change to dark

  Scenario: Keyboard help button shows shortcuts
    Given the application is running
    When I press F1
    Then the keyboard help modal should open

  Scenario: Logo is displayed in top bar
    Given the application is running
    When I view the top bar
    Then the application logo should be visible

  # ==================== Search and Filter ====================

  Scenario: Search filters audiobooks by title
    Given the application has audiobooks "Adventure" and "Mystery"
    When I search for "Adv"
    Then only "Adventure" should be displayed

  Scenario: Search filters audiobooks by author
    Given the application has audiobooks by "Jane" and "John"
    When I search for "Jane"
    Then only audiobooks by Jane should be displayed

  Scenario: Clear search shows all audiobooks
    Given the application has 3 audiobooks
    And I search for "xyz"
    When I clear the search
    Then all 3 audiobooks should be displayed

  Scenario: Sort by title orders alphabetically
    Given the application has audiobooks "Zebra" and "Apple"
    When I sort by title ascending
    Then "Apple" should be listed before "Zebra"

  Scenario: Sort by last listened shows recent first
    Given the application has audiobooks with different last listened dates
    When I sort by last listened
    Then the most recent should be first

  # ==================== Settings Modal ====================

  Scenario: Theme preference is stored
    Given the application has a settings store
    When I set a metadata value
    Then the metadata value should be stored

  Scenario: Settings modal has theme dropdown
    Given the settings modal is open
    When I view the theme options
    Then Light, Dark, and System options should be available

  Scenario: Settings modal has directory management
    Given the settings modal is open
    When I view the directories section
    Then an add directory button should be visible

  Scenario: Settings modal has skip duration controls
    Given the settings modal is open
    When I view the playback section
    Then skip forward duration options should be visible
    And skip backward duration options should be visible

  Scenario: Settings modal has focus mode toggle
    Given the settings modal is open
    When I view the accessibility section
    Then a focus mode toggle should be visible

  Scenario: Settings modal has reduced motion toggle
    Given the settings modal is open
    When I view the accessibility section
    Then a reduced motion toggle should be visible

  Scenario: Settings modal has reset to defaults button
    Given the settings modal is open
    And some settings have been changed
    When I click reset to defaults
    Then all settings should return to default values

  Scenario: Close button closes settings modal
    Given the settings modal is open
    When I click the close button
    Then the modal should close

  Scenario: Done button saves and closes settings modal
    Given the settings modal is open
    When I click the done button
    Then the modal should close
    And settings should be saved

  # ==================== Sleep Timer Modal ====================

  Scenario: Sleep timer modal has duration options
    Given the sleep timer modal is open
    When I view the duration options
    Then 15, 30, 60, and 90 minute options should be available

  Scenario: Sleep timer modal has end of chapter option
    Given the sleep timer modal is open
    When I view the options
    Then an end of chapter option should be available

  Scenario: Sleep timer modal has fade duration setting
    Given the sleep timer modal is open
    When I view the fade options
    Then fade duration options should be available

  Scenario: Active sleep timer shows remaining time
    Given a sleep timer is active with 30 minutes remaining
    When I view the player controls
    Then the remaining time should be displayed

  Scenario: Cancel sleep timer button works
    Given a sleep timer is active
    When I click cancel timer
    Then the timer should be cancelled
    And no remaining time should be displayed

  # ==================== Directory Management ====================

  Scenario: Add directory button opens file picker
    Given the settings modal is open
    When I click add directory
    Then a file picker should open

  Scenario: Directory list shows all directories
    Given 2 directories have been added
    When I view the directories section
    Then 2 directories should be listed

  Scenario: Remove directory removes from list
    Given 2 directories have been added
    When I remove the first directory
    Then only 1 directory should remain

  Scenario: Rescan directory triggers scan
    Given a directory has been added
    When I click rescan
    Then a scan should start
    And a loading indicator should appear

  # ==================== Panel Resize ====================

  Scenario: Panel divider is visible
    Given the application is running
    When I view the layout
    Then a divider between panels should be visible

  Scenario: Dragging divider resizes panels
    Given the application is running
    When I drag the divider to the right
    Then the left panel should be wider
    And the right panel should be narrower

  Scenario: Panel ratio is persisted
    Given I have resized the panels to 60/40
    When I restart the application
    Then the panels should remain at 60/40

  # ==================== Focus Mode ====================

  Scenario: Focus mode can be toggled
    Given the application has a settings store
    When I enable focus mode
    Then focus mode should be enabled
    When I disable focus mode
    Then focus mode should be disabled

  Scenario: Focus mode hides non-essential UI
    Given focus mode is enabled
    When I view the interface
    Then non-essential elements should be hidden

  # ==================== Loading States ====================

  Scenario: Loading spinner shows during data fetch
    Given the application is loading audiobooks
    When I view the library
    Then a loading spinner should be visible

  Scenario: Skeleton placeholders show during load
    Given the application is loading
    When I view the library
    Then skeleton placeholders should be displayed

  Scenario: Loading completes and shows content
    Given the application is loading
    When loading completes
    Then the actual content should be displayed

  # ==================== Error States ====================

  Scenario: Error message shows when loading fails
    Given loading audiobooks fails
    When I view the library
    Then an error message should be displayed

  Scenario: Retry button appears on error
    Given loading audiobooks fails
    When I view the error state
    Then a retry button should be available

  Scenario: Retry button retries the operation
    Given loading audiobooks fails
    And a retry button is visible
    When I click retry
    Then the operation should be retried

  # ==================== Keyboard Shortcuts ====================

  Scenario: Space key toggles play/pause
    Given the application is running
    And a file is loaded
    When I press Space
    Then playback should toggle

  Scenario: M key toggles mute
    Given the application is running
    And a file is playing
    When I press M
    Then mute should toggle

  Scenario: Arrow keys seek
    Given the application is running
    And a file is playing
    When I press ArrowRight
    Then the position should advance
    When I press ArrowLeft
    Then the position should rewind

  Scenario: Escape closes modal
    Given a modal is open
    When I press Escape
    Then the modal should close

  Scenario: Ctrl+Comma opens settings
    Given the application is running
    When I press Ctrl+Comma
    Then the settings modal should open

  Scenario: Ctrl+B creates bookmark
    Given the application is running
    And a file is playing
    When I press Ctrl+B
    Then a bookmark should be created

  Scenario: Ctrl+T opens sleep timer
    Given the application is running
    When I press Ctrl+T
    Then the sleep timer modal should open

  Scenario: F1 opens keyboard help
    Given the application is running
    When I press F1
    Then the keyboard help modal should open

  # ==================== Accessibility ====================

  Scenario: All buttons have aria-labels
    Given the application is running
    When I check accessibility
    Then all buttons should have aria-labels

  Scenario: Focus is visible on all elements
    Given the application is running
    When I tab through elements
    Then focus indicators should be visible

  Scenario: Modal traps focus
    Given a modal is open
    When I press Tab multiple times
    Then focus should remain within the modal

  Scenario: Screen reader announcements for state changes
    Given screen reader is active
    When playback starts
    Then a screen reader announcement should be made

  Scenario: Color contrast meets WCAG standards
    Given the application is running
    When I check color contrast
    Then all text should have sufficient contrast

  # ==================== Cover Art ====================

  Scenario: Cover art displays for audiobook
    Given the application has an audiobook with cover art
    When I view the audiobook
    Then the cover art should be displayed

  Scenario: Placeholder shows when no cover art
    Given the application has an audiobook without cover art
    When I view the audiobook
    Then a placeholder image should be displayed

  Scenario: Cover art in details panel
    Given the application has an audiobook with cover art
    When I select the audiobook
    Then the cover art should appear in the details panel

  # ==================== Progress Tracking ====================

  Scenario: Audiobook progress is displayed
    Given the application has an audiobook at 50% completion
    When I view the audiobook
    Then 50% progress should be displayed

  Scenario: Remaining time is calculated
    Given the application has an audiobook with 2 hours remaining
    When I view the audiobook
    Then 2 hours remaining should be displayed

  Scenario: Completion badge shows for finished books
    Given the application has a completed audiobook
    When I view the audiobook
    Then a completion indicator should be visible

  # ==================== Notifications ====================

  Scenario: Success notification displays
    Given an operation succeeds
    When a notification is triggered
    Then a success notification should appear

  Scenario: Error notification displays
    Given an operation fails
    When a notification is triggered
    Then an error notification should appear

  Scenario: Notification dismisses automatically
    Given a notification is showing
    When the duration expires
    Then the notification should disappear

  Scenario: Notification can be dismissed manually
    Given a notification is showing
    When I click the dismiss button
    Then the notification should disappear

  # ==================== Delete Confirmation ====================

  Scenario: Delete audiobook shows confirmation dialog
    Given the application has an audiobook
    And the audiobook has files
    When I click delete audiobook
    Then a confirmation dialog should appear

  Scenario: Delete audiobook can be cancelled
    Given the application has an audiobook
    When I click delete audiobook
    And I cancel the deletion
    Then the audiobook count should be one

  Scenario: Confirming deletion removes audiobook
    Given the application has an audiobook
    And the audiobook has files
    When I click delete audiobook
    And I confirm the deletion
    Then the audiobook count should be zero

  # ==================== Bulk Selection ====================

  Scenario: Bulk selection of audiobooks
    Given the application has 5 audiobooks
    When I select multiple audiobooks with Ctrl+Click
    Then all selected audiobooks should be highlighted
    And the bulk action bar should appear

  Scenario: Bulk delete audiobooks
    Given the application has 5 audiobooks
    And I select multiple audiobooks with Ctrl+Click
    When I click bulk delete
    Then a confirmation dialog should appear
    And the selected count should be shown

  Scenario: Select all audiobooks
    Given the application has 5 audiobooks
    When I press Ctrl+A
    Then all audiobooks should be selected
