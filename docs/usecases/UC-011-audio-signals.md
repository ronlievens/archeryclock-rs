# UC-011: Audio Signals and Buzzer

## User Story
**As an** Operator  
**I want** audio signals (beeps/WAV) and buzzer at phase changes  
**So that** archers hear timing signals

## Preconditions
- Audio configured

## Acceptance Criteria
```gherkin
Feature: Audio signals

  Scenario: Play sounds per phase
    Given sounds assigned to states/phases
    When phase changes
    Then appropriate WAV/MP3 played via SDL_mixer or system audio

  Scenario: Configure volumes
    Given volume settings per state
    When state changes
    Then volume applied

  Scenario: Hardware buzzer
    Given buzzer enabled (Buzzer mode)
    When signals occur
    Then hardware buzzer triggered if available (K8055/GPIO/external)

  Scenario: Remote audio control
    Given remote audio enabled (VLC/MP3/network)
    When configured
    Then send commands to remote audio player

  Scenario: Test sounds
    Given sound selected
    When operator tests
    Then sound plays
```

## Technical Notes
- `sounds/` directory with WAV/MP3 files
- `soundselect`, per-state audio settings
- `Controlaudiocheckbox`, `TCPmusiccntrl` for remote
- `VLCradiobutton`, `MP3_2VLCradiobutton`
- Buzzer modes: Buzzer/speaker settings

## GUI Wireframe (Sound)

```text
Sound tab: Master volume, Per-state sounds [WAV▼][Test],
Buzzer/Speaker, Remote audio (VLC/MP3/Network), Beeps config
```
