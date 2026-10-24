# UC-003: Configure FITA Timing and Sequences

## User Story
**As an** Operator  
**I want** to configure FITA timing (green/orange/red), ends/turns, archer sequences and alternation  
**So that** it matches the event format

## References
- World Archery Rulebook (FITA/WA formats) - https://www.worldarchery.sport/rulebook

## Preconditions
- Format is FITA (UC-002)

## Acceptance Criteria
```gherkin
Feature: Configure FITA timing

  Scenario: Set per-end times
    Given FITA mode active
    When operator sets green/orange/red times
    Then times are stored and used for shoot/warning/last phases

  Scenario: Configure archer groups and sequence
    Given FITA mode
    When operator selects group pattern (1x1,2x1,AB-CD, etc.) and ABCD per end
    Then sequence is calculated (fitaseq/fitaseq2) with correct alternation
    And display shows correct archer order

  Scenario: Configure practice vs competition
    Given FITA mode
    When operator sets practice turns count
    Then practice timing/behavior applied for practice turns

  Scenario: Configure shoot-off times
    Given FITA mode with shoot-off possible
    When operator configures shoot-off green/orange/red
    Then shoot-off phases use shoot-off times
```

## Technical Notes
- Times: `greentime/orangetime/redtime`, `greentimeso/orangetimeso/redtimeso`
- Patterns: topbot, oneone, onetwo, onethree, onefour, onefive, onesix, twotwo, twothree, threetwo
- `ABCDperEND` controls alternation
- `nrofturns`, `turnsss`, `times` affect sequence display

## GUI Wireframe (FITA timing)

```text
FITA panel: Group pattern [1x1/2x1/AB-CD/...], ABCD per end [✓],
Green/Orange/Red [ 90/30/10 ], Practice turns [2], SO times [...]
```
