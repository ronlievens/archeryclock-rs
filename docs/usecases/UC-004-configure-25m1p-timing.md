# UC-004: Configure 25m 1+1P Timing and Rotation

## User Story
**As an** Operator  
**I want** to configure 25m 1+1P settings (times, shooters, rotation)  
**So that** it matches 25m Precision/Rapid format

## References
- World Archery Rulebook - 25m formats - https://www.worldarchery.sport/rulebook

## Preconditions
- Format is 25m 1+1P (UC-002)

## Acceptance Criteria
```gherkin
Feature: Configure 25m 1+1P

  Scenario: Set 25m1P times
    Given 25m1P mode
    When operator sets green/orange/red times for 1+1P
    Then `greentime1p/orangetime1p/redtime1p` used

  Scenario: Configure shooters and rotation
    Given 25m1P mode
    When operator sets number of shooters and rotation options (`rotate25m1p`, `rotate25m1p_p`)
    Then archer rotation/display behaves correctly
    And per-arrow numbering shown as appropriate

  Scenario: Configure 25m1P shoot-off
    Given shoot-off in 25m1P
    When shoot-off times configured
    Then shoot-off uses correct times

  Scenario: ABCD/archer display for 25m1P
    Given 25m1P with multiple archers
    When configured
    Then archer labels and rotation update correctly
```

## Technical Notes
- `archerysystem='25m1p'`
- Variables: `nrof1pshooters`, `greentime1p`, `orangetime1p`, `redtime1p`, `rotate25m1p`, `rotate25m1p_p`, `abcd25m1p`
- Special display logic for 25m1P (per-arrow vs per-end)

## GUI Wireframe (25m 1+1P)

```text
25m1P panel: Shooters [4], Rotation [✓ rotate25m1p],
Times 1+1P G/O/R [...], SO times [...], Archer numbering
```
