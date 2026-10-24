# UC-009: Emergency Stop / Pause

## User Story
**As a** Judge/Operator  
**I want** to trigger emergency stop/pause  
**So that** shooting can be halted immediately

## Preconditions
- Clock running

## Acceptance Criteria
```gherkin
Feature: Emergency

  Scenario: Trigger emergency
    Given running sequence
    When operator triggers emergency
    Then state becomes emergency (6)
    And emergency indication shown (text/color)
    And configured action taken (stop current / stop next / pause)

  Scenario: Emergency behavior configuration
    Given emergency settings
    When configured behavior is "Stop current"
    Then current phase stopped
    When "Stop next"
    Then will stop at next opportunity
    When "Pause"
    Then paused

  Scenario: Emergency signals
    Given emergency triggered
    Then emergency sound/beeps played per configuration
    And buzzer triggered if enabled

  Scenario: Resume from emergency
    Given in emergency state
    When operator resumes
    Then returns to appropriate state/behavior
```

## Technical Notes
- `EmergencydefaultRadioButton` / `EmergencyalternativeRadioButton`, `Pause_emergency_RadioButton`, `Stop_current_RadioButton`, `Stop_next_RadioButton`
- `EmergencytxtEdit`, `EmergencysoundGroupBox`
- `KeepbuzzingCheckBox`, `nr*beep` settings

## GUI Wireframe (Emergency)

```text
Emergency tab/panel: Behavior [Stop current/Stop next/Pause],
Emergency text, Sound, Beeps, Keep buzzing [✓],
Big EMERGENCY indication on display
```
