# UC-015: Access Point Mode (Linux/RPi)

## User Story
**As an** Operator  
**I want** to run the clock as WiFi access point on Linux/RPi  
**So that** clients can connect without external network

## Preconditions
- Running on Linux/RPi with wireless interface
- Sufficient permissions (sudo) for network config

## Acceptance Criteria
```gherkin
Feature: Access point

  Scenario: Configure AP settings
    Given AP settings available
    When operator sets SSID, password, channel, country, interface
    Then settings stored

  Scenario: Enable access point
    Given AP configured
    When operator enables AP
    Then hostapd/dhcpcd configured and services started (writes configs like hostapd.conf, dhcpcd.conf)

  Scenario: Disable access point
    Given AP running
    When operator disables
    Then AP stopped and original network config restored

  Scenario: AP status
    Given AP state
    Then status shown
```

## Technical Notes
- Uses `hostapd`, `dhcpcd`, copies config files from app dir
- `accessp` file format (see orginal-src/accessp)
- `enableaccesspointbutton`, `disableaccesspointbutton1`
- Linux only in practice

## GUI Wireframe (Access Point)

```text
AP panel (Linux): Enable AP [✓], SSID [ArcheryClock], Password, Channel, Country, Interface,
[Enable AP] [Disable AP] Status
```
