# UC-017: Configuration Persistence

## User Story
**As an** Operator  
**I want** settings and scenarios to be saved/loaded  
**So that** configuration persists between runs

## Preconditions
- Application has write access to config directory

## Acceptance Criteria
```gherkin
Feature: Configuration persistence

  Scenario: Save settings
    Given settings changed
    When operator saves
    Then settings written to settings file in correct format

  Scenario: Load settings
    Given settings file exists
    When starting
    Then settings loaded in correct order

  Scenario: Save scenario
    Given scenario configured
    When saved
    Then written to scenarios/Fx

  Scenario: Load scenario
    Given scenario file exists
    When loaded
    Then applied correctly

  Scenario: Compatible format
    Given original settings/scenario format
    Then can be read (for migration) and/or new format supported
```

## Technical Notes
- Settings file format is line-based (see original)
- Scenarios similar line-based format
- For Rust implementation, consider TOML for new format but support reading original format

## GUI Wireframe (Config)

```text
Settings saved to 'settings' file, Scenarios saved to 'scenarios/Fx'
UI shows [Save] in Settings panel
```
