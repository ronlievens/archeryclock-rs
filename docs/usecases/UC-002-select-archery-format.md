# UC-002: Select Archery Format/System

## User Story
**As an** Operator  
**I want** to select the competition format (FITA, FITA Finals, 25m 1+1P, Manual, Follow)  
**So that** the clock uses correct timing rules, UI, and behavior

## Preconditions
- Application started (UC-001)

## Acceptance Criteria
```gherkin
Feature: Select archery format

  Scenario: Select FITA system
    Given no format selected or different format
    When operator selects FITA
    Then archerysystem = "fita"
    And FITA-specific controls are shown
    And default FITA times/sequence options are applied
    And display/layout adapts to FITA mode

  Scenario: Select FITA Finals
    Given operator wants shoot-off/finals
    When operator selects FITA Finals (fitafin)
    Then archerysystem = "fitafin"
    And finals controls shown (arrow/target, choose archer)
    And finals timing options enabled

  Scenario: Select 25m 1+1P
    When operator selects 25m 1+1P
    Then archerysystem = "25m1p"
    And per-arrow/rotation controls shown
    And 25m1P timing defaults applied

  Scenario: Select Manual
    When operator selects Manual
    Then archerysystem = "manual"
    And manual control mode enabled

  Scenario: Select Follow (client)
    When operator selects Follow
    Then archerysystem = "follow"
    And follow system type mapping applies based on host
    And client sync behavior enabled
```

## Technical Notes
- `archerysystem` in {"fita","fitafin","25m1p","manual","follow"}
- `followsystem` indicates source type in follow mode
- `initsystemtype()` applies format-specific settings
- UI elements shown/hidden based on format

## GUI Wireframe (format selection)

```text
System selector: [FITA▼] / [FITA Finals▼] / [25m 1+1P▼] / [Manual▼] / [Follow▼]
Contextual format panels appear below menu.
```
