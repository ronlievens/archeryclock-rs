# UC-005: Configure FITA Finals / Shoot-Off

## User Story
**As an** Operator  
**I want** to configure FITA Finals mode (arrow/target, archer selection, finals timing)  
**So that** shoot-offs and finals run correctly

## References
- World Archery Rulebook (Finals) - https://www.worldarchery.sport/rulebook

## Preconditions
- Format is FITA Finals (`fitafin`)

## Acceptance Criteria
```gherkin
Feature: Configure FITA Finals

  Scenario: Select arrow-based or target-based timing
    Given fitafin mode
    When operator selects arrow-based (`finarrow`) or target-based (`fintarget`)
    Then timing mode set and UI updates accordingly

  Scenario: Choose starting archer
    Given fitafin mode
    When operator chooses archer (left/right/choose)
    Then `fitafinarcher` set and display shows correct archer
    And hints displayed if needed

  Scenario: Configure finals times
    Given fitafin mode
    When operator sets finals times (per arrow/target) and shoot-off times
    Then correct times used for finals/shoot-off

  Scenario: Time per end vs time per turn
    Given fitafin mode
    When operator toggles `timeperend`/`timeperturn`
    Then timing behavior changes accordingly
```

## Technical Notes
- `archerysystem='fitafin'`, `finarrow`, `fintarget`, `fitafinarcher`, `timeperend`, `timeperturn`
- `greenfinal/orangefinal/redfinal`, `greenfinalend/orangefinalend/redfinalend`
- Complex view logic for finals display

## GUI Wireframe (FITA Finals)

```text
Finals panel: Mode [Arrow | Target], Choose archer [Left/Right/Choose],
Time per end/turn, Finals/SO times, Arrows/Target controls
```
