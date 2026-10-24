# UC-008: Run Countdown Sequence (Normal Operation)

## User Story
**As a** Timekeeper  
**I want** to run the countdown sequence through states (wait/green/orange/red/emergency/pre-count)  
**So that** archers are timed correctly

## Preconditions
- Format configured (UC-002)
- Timing configured (UC-003/UC-004/UC-005)
- Clock ready

## Acceptance Criteria
```gherkin
Feature: Countdown sequence

  Scenario: Start sequence
    Given ready state
    When operator triggers start/next
    Then state transitions from wait/pre to green (shoot)
    And timers start with correct time for current phase

  Scenario: Phase transitions
    Given in green phase
    When time reaches orange threshold
    Then state becomes orange, colors/signals updated
    When time reaches red threshold
    Then state becomes red/last seconds, signals updated

  Scenario: End of phase / advance
    Given phase ends
    When operator advances or auto-advance occurs
    Then move to next archer/end/turn according to format/sequences
    And state resets to wait/green as appropriate

  Scenario: Pre-count to start
    Given pre-count enabled
    When starting match
    Then countdown-to-start shown before first phase

  Scenario: Track ends/turns/series
    Given running match
    Then end/turn/series numbers update correctly per format
```

## Technical Notes
- States: 1 wait, 2 green, 3 orange, 4 red, 5 countdown to start, 6 emergency
- `state`, `realstate`, `prevstate`
- Timers: `SecondTimer`, phase timers
- Sequence logic varies by format

## GUI Wireframe (main clock display)

```text
(see main display above) Left/Right sides, big time digits MM:SS, colors by state,
End/Turn/Series, Archer labels, Traffic lights optional
```
