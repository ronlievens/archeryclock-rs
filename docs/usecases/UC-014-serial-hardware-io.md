# UC-014: Serial, Arduino, K8055, GPIO/Hardware I/O

## User Story
**As an** Operator  
**I want** to use external hardware inputs (serial, Arduino, K8055, GPIO, remotes)  
**So that** I can control the clock without keyboard

## Preconditions
- Hardware connected/configured

## Acceptance Criteria
```gherkin
Feature: Hardware I/O

  Scenario: Configure serial ports
    Given serial devices connected
    When operator selects ports (COM*/tty*)
    Then serial communication initialized

  Scenario: Arduino input
    Given Arduino enabled
    When signals received from Arduino
    Then mapped to actions

  Scenario: K8055 input (Windows)
    Given K8055 enabled on Windows
    When buttons pressed
    Then mapped to actions

  Scenario: GPIO/RPi inputs
    Given on Linux/RPi with GPIO
    When inputs triggered
    Then mapped to actions

  Scenario: Button/key mapping
    Given input mapping configured
    When external input occurs
    Then correct action performed (start/next/stop/emergency)

  Scenario: Laser pointers/presentation remotes
    Given mapped
    Then work as control inputs
```

## Technical Notes
- `serial`, `serial2` (SdpoSerial), `K8055select`, `K8055D.dll`
- `HWbutton1-5`, mappings
- `arduino1`, `arduino2`
- Platform specific

## GUI Wireframe (Hardware/IO)

```text
HW/IO tab: Serial Port1/2 [COM*/tty*], Arduino [✓], K8055 [✓],
GPIO/RPi mapping, Keyboard/Remote/Laser pointer mapping, Button mapping
```
