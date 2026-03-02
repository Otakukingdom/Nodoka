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

  Scenario: Invalid image is skipped and fallback is used
    Given an audiobook directory with corrupted folder.jpg
    And the directory contains valid cover.png
    When I select cover art
    Then cover.png should be selected

