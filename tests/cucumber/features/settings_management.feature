Feature: Settings Management
  As an audiobook listener
  I want to customize my listening experience
  So that the application works according to my preferences

  Scenario: Theme preference is persisted
    Given the application has a settings store
    When I set the theme to Dark
    Then the theme should be stored as Dark

  Scenario: Volume preference is persisted
    Given the application has a settings store
    When I set the volume to 75
    Then the volume should be stored as 75

  Scenario: Speed preference is persisted
    Given the application has a settings store
    When I set the speed to 1.5
    Then the speed should be stored as 1.5

  Scenario: Panel ratio is persisted
    Given the application has a settings store
    When I set the panel ratio to 0.5
    Then the panel ratio should be stored as 0.5

  Scenario: Focus mode can be enabled
    Given the application has a settings store
    When I enable focus mode
    Then focus mode should be enabled

  Scenario: Focus mode can be disabled
    Given the application has a settings store
    When I enable focus mode
    And I disable focus mode
    Then focus mode should be disabled

  Scenario: Reduced motion preference is persisted
    Given the application has a settings store
    When I enable prefers reduced motion
    Then the reduced motion preference should be stored as true

  Scenario: All settings can be retrieved
    Given the application has a settings store
    And I set the theme to Dark
    And I set the volume to 75
    When I retrieve settings
    Then all settings should be available
