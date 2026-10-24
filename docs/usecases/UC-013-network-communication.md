# UC-013: Network Communication (Host/Client/Control)

## User Story
**As an** Operator  
**I want** network communication for host/client sync and remote control  
**So that** clocks can be networked

## Preconditions
- Network configured

## Acceptance Criteria
```gherkin
Feature: Network communication

  Scenario: Configure host/client
    Given network settings
    When operator sets LAN host/client mode, IP/multicast/broadcast, ports
    Then networking initialized

  Scenario: Broadcast state (host)
    Given in host mode
    When state changes
    Then state broadcast over network

  Scenario: Receive state (client)
    Given in client/follow mode
    When packets received
    Then state updated

  Scenario: TCP control
    Given TCP configured
    Then remote commands can be received

  Scenario: Multicast address
    Given default multicast 224.25.26.27
    When configured
    Then join/send on correct multicast group

  Scenario: Network timestamp/sync
    Given network active
    Then timestamps handled for sync
```

## Technical Notes
- `LUDP1`, `TCPaccess1`, `TCPmusiccntrl` in original (lNet)
- Settings: network mode, `portnumber1`, multicast IP parts, `fromHOST`
- Default multicast in settings: 224,25,26,27 port 4665

## GUI Wireframe (Network)

```text
Network tab: Mode [OFF/LAN Host/LAN Client], Multicast [224.25.26.27:4665],
TCP, Receive from host, Remote shutdown
```
