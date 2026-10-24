# UC-006: Manual Mode Operation

## User Story
**As an** Operator  
**I want** to run the clock in manual mode with full control over phases  
**So that** I can time non-standard situations

## Preconditions
- Format is Manual (`manual`)

## Acceptance Criteria
```gherkin
Feature: Manual mode

  Scenario: Enter manual mode
    Given operator selects Manual
    Then manual controls enabled
    And automatic end/turn progression may be disabled

  Scenario: Manually control phases
    Given in manual mode
    When operator triggers phase changes (start/next/stop)
    Then phases change as directed
    And colors/signals update

  Scenario: Manual timing
    Given in manual mode
    When operator controls timing
    Then countdown behaves according to manual control
```

## Technical Notes
- `archerysystem='manual'`
- Traffic/signaling behavior differs

## GUI Wireframe (Manual)

```text
Manual panel: [START/NEXT] [STOP] [PAUSE] [EMERGENCY] - full manual control
```
