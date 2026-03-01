Feature: UI Component Behavior
  As an audiobook listener
  I want a responsive and accessible user interface
  So that I can efficiently interact with the application

  Scenario: Application loads and displays library
    Given the application has an audiobook
    When I request all audiobooks
    Then the audiobook count should be one

  Scenario: Selecting an audiobook displays files
    Given the application has an audiobook
    And the audiobook has files
    When I request the files for the audiobook
    Then I should receive three files
    And the files should be ordered by position

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

  Scenario: Theme preference is stored
    Given the application has a settings store
    When I set a metadata value
    Then the metadata value should be stored

  Scenario: Focus mode can be toggled
    Given the application has a settings store
    When I enable focus mode
    Then focus mode should be enabled
    When I disable focus mode
    Then focus mode should be disabled

  Scenario: Bookmarks can be displayed for selected audiobook
    Given the application has an audiobook
    And the audiobook has bookmarks
    When I request the bookmarks for the audiobook
    Then I should receive three bookmarks
