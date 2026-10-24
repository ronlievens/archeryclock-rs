# UC-007: Follow Mode (Network Client)

## User Story
**As an** Operator  
**I want** the clock to follow a remote host over network  
**So that** multiple displays show synchronized state

## Preconditions
- Network configured
- Follow mode selected or host-client configured

## Acceptance Criteria
```gherkin
Feature: Follow mode

  Scenario: Configure follow/client settings
    Given network tab configured
    When operator sets follow/LAN client mode, host IP/port, multicast/broadcast
    Then client listens for host state

  Scenario: Sync state from host
    Given client connected to host
    When host state changes (phase/time/format/end/turn)
    Then client updates display/state to match
    And followsystem mapping applied

  Scenario: Follow system type mapping
    Given follow mode
    When host system type received
    Then followsystem set appropriately (maps to fita/fitafin/25m1p/manual variants)
    And UI adapts

  Scenario: Disconnect/reconnect
    Given network issues
    When connection lost/restored
    Then client handles reconnect according to settings
```

## Technical Notes
- `archerysystem='follow'`, `followsystem` ('0'-'7' etc.)
- Uses UDP multicast/broadcast/TCP (lNet in original), default multicast 224.25.26.27:4665
- `Lanremoteselect`, `LANselect`, `LANinput` settings
- State synced via network protocol (broadcast strings)

## GUI Wireframe (Follow/Client)

```text
Network panel (client): Mode [Follow/LAN Client], Host IP/Port/Multicast,
Follow system mapping shown, Status [Connected/Disconnected]
```
