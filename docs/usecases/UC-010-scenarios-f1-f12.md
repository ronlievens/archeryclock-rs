# UC-010: Scenarios (F1-F12) - Program and Run

## User Story
**As an** Operator  
**I want** to program, save, load and run scenarios (F1-F12)  
**So that** I can quickly switch between common competition setups

## Preconditions
- Application started

## Acceptance Criteria
```gherkin
Feature: Scenarios

  Scenario: Initialize scenarios at startup
    Given scenarios may exist in scenarios/ or not
    When starting
    Then for F1-F12, load from file if exists else use defaults
    And scenario info shown in UI

  Scenario: Load scenario
    Given scenario file exists (F1-F12 or custom)
    When operator loads scenario
    Then `getscenario()` reads file and `getsetting()` applies settings
    And format/times/sequences updated

  Scenario: Save scenario
    Given current settings configured
    When operator saves scenario
    Then scenario written to scenarios/ file

  Scenario: Run scenario via F-key/startup
    Given scenario selected (startup or F-key)
    When activated
    Then scenario applied and can be run

  Scenario: Scenario file format
    Given scenario file (e.g. orginal-exe/scenarios/gen)
    When read
    Then contains format and all relevant settings in order (see parsed gen)
```

## Technical Notes
- Scenario files in `scenarios/` directory
- `scenario` variable holds name (F1-F12)
- `getscenario()`, `showF1()`..`showF12()`, `Setsettings()`
- `startupscenario` setting
- See `orginal-exe/scenarios/gen` for format example

## GUI Wireframe (Scenarios)

```text
Scenario tab: Startup [--/F1-F12], F1-F12 buttons with descriptions,
[Load] [Save] [Edit] per scenario, Times/sequences shown
```
