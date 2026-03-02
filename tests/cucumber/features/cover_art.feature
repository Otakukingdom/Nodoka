@ui
Feature: Cover Art Selection
  As an audiobook listener
  I want cover art to be automatically selected
  So that my audiobooks display the correct artwork

  @critical
  Scenario: folder.jpg selected when no embedded image exists
    Given an audiobook directory without embedded cover art
    And the directory contains folder.jpg
    When I select cover art
    Then folder.jpg should be selected

  Scenario: cover.jpg selected as fallback
    Given an audiobook directory without embedded cover art
    And the directory contains no folder.jpg
    And the directory contains cover.jpg
    When I select cover art
    Then cover.jpg should be selected

  Scenario: cover.png selected as last resort
    Given an audiobook directory without embedded cover art
    And the directory contains no folder.jpg or cover.jpg
    And the directory contains cover.png
    When I select cover art
    Then cover.png should be selected

  Scenario: No cover art when no candidates exist
    Given an audiobook directory without any cover art
    When I select cover art
    Then no cover art should be selected

  # ==================== UI Display Scenarios ====================

  @ui
  Scenario: Cover art displays in library view
    Given the application has an audiobook with cover art
    When I view the library
    Then the cover art should be displayed in the card

  Scenario: Cover art displays in details panel
    Given the application has an audiobook with cover art
    When I select the audiobook
    Then the cover art should appear in the details panel

  Scenario: Placeholder shows when cover not loaded
    Given the application has an audiobook without cover art
    When I view the audiobook
    Then a placeholder image should be displayed

  Scenario: Cover art updates when file changes
    Given the application has an audiobook with cover art
    And the cover art file is modified
    When I rescan the directory
    Then the new cover art should be displayed

  Scenario: Cover art aspect ratio is preserved
    Given the application has an audiobook with cover art
    When I view the audiobook
    Then the cover art should maintain aspect ratio

