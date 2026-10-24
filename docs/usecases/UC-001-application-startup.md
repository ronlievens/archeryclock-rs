# UC-001: Application Startup and Initialization

## User Story
**As an** Operator  
**I want** the ArcheryClock application to start up, load configuration, assets and initialize I/O  
**So that** the clock is ready to use in the configured mode

## Scope
Startup sequence from original: load `settings`, load language, initialize UI, timers, serial/network/hardware, load scenarios, apply startup scenario.

## Preconditions
- Application binary and assets present (sounds/, images, language/, scenarios/ if used)
- Optional: config files `settings`, language files

## Acceptance Criteria
```gherkin
Feature: Application startup

  Scenario: First run with no settings file
    Given no "settings" file exists
    When the application starts
    Then default settings are written (language, time format, icons, buzzer, ports, colors, banners, music, etc.)
    And default UI state is initialized
    And application enters ready state

  Scenario: Normal startup with settings
    Given "settings" file exists
    When the application starts
    Then settings are read in order and applied (language, min/sec, hide icons, sound mode, startup scenario, traffic lights, ports, network, colors, banner, fullscreen/slides, music settings)
    And language file is loaded
    And serial port lists are populated per platform (COM*/tty*)
    And scenarios F1-F12 are initialized from files or defaults
    And startup scenario is applied if configured
    And display is shown according to configuration

  Scenario: Platform-specific initialization
    Given running on Windows or Linux
    When initializing
    Then platform-appropriate defaults/paths are used (serial ports, K8055 available on Windows, GPIO/AP on Linux)
```

## Technical Notes (from source)
- `Filespath` resolved at startup
- Reads settings line by line (see `FormCreate` / initialization)
- Loads language via `setlanguage()`
- `getscenario()` / `Setsettings()` for scenarios
- Traffic lights, colors, music/audio control initialized

## GUI Wireframe (from original UI)

```text
ARCHERY CLOCK - GUI Wireframe (concept based on original)

┌─────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│ ArcheryClock [FITA / 25m1P / Fitafin / Manual / Follow]                        [ _ ] [ □ ] [ X ] │
├─────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ Menu/Icons: [Scenario] [PreCount] [Colors] [Sound] [Network] [HW/IO] [Emergency] [Lang] [About]    │
└─────────────────────────────────────────────────────────────────────────────────────────────────────────┘

MAIN CLOCK DISPLAY (fullscreen/split view)
┌─────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                                                                                         │
│  LEFT SIDE                                    │  RIGHT SIDE                                      │
│  ┌─────────────────────────────┐             │  ┌─────────────────────────────┐                │
│  │ Archer: A / LEFT           │             │  │ Archer: B / RIGHT          │                │
│  │ End/Turn: E1 T1            │             │  │ End/Turn: E1 T1            │                │
│  │        ┌───────┐           │             │  │        ┌───────┐           │                │
│  │        │  02:15 │  (MM:SS) │             │  │        │  02:15 │           │                │
│  │        └───────┘           │             │  │        └───────┘           │                │
│  │ State: GREEN/ORANGE/RED    │             │  │ State: GREEN/ORANGE/RED    │                │
│  └─────────────────────────────┘             │  └─────────────────────────────┘                │
│                                                                                                         │
│                        CENTER INFO: SERIES/END NR / ARROWS / PRACTICE                              │
│                        BANNER / BACKGROUND / COMMERCIAL SLIDES                                     │
└─────────────────────────────────────────────────────────────────────────────────────────────────────────┘

CONTROLS (operator)
┌─────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│ [START/NEXT] [STOP] [PAUSE] [EMERGENCY]  │  Scenario: [F1▼]  │  System: [FITA▼]  │  [SETTINGS]     │
└─────────────────────────────────────────────────────────────────────────────────────────────────────────┘

SETTINGS PANELS (detailed list of options - to be made user-friendly later)

1) SCENARIO TAB
   ┌─────────────────────────────────────────────────────────────────────────────────────────┐
   │ Startup Scenario: [-- / F1-F12]                                                        │
   │ Scenario List/Editor: [Load] [Save] [Edit]                                              │
   │ F-Keys Programmable (F1-F12):                                                          │
   │  F1: [Desc] Times(G/O/R) SO(G/O/R) Seq[...]                                             │
   │  ...                                                                                    │
   └─────────────────────────────────────────────────────────────────────────────────────────┘

2) PRE-COUNT TAB
   ┌─────────────────────────────────────────────────────────────────────────────────────────┐
   │ Pre-count to start: [ON/OFF]  Time: [10] sec                                           │
   │ Show date/time or slides                                                                 │
   └─────────────────────────────────────────────────────────────────────────────────────────┘

3) SCREEN COLORS TAB
   ┌─────────────────────────────────────────────────────────────────────────────────────────┐
   │ Color schemes: [1][2][3][4][5][6][7][8][9]                                             │
   │ Per state: Preparation/Green/Orange/Red/Retrieve/Background                           │
   │ Time color, Background color, AB/CD colors                                             │
   └─────────────────────────────────────────────────────────────────────────────────────────┘

4) SOUND TAB
   ┌─────────────────────────────────────────────────────────────────────────────────────────┐
   │ Master volume: [====]                                                                   │
   │ Per state sounds: [Select WAV] [Test]                                                  │
   │ Buzzer: [ON/OFF] Beeps config                                                          │
   │ Remote audio (VLC/MP3/Network)                                                          │
   └─────────────────────────────────────────────────────────────────────────────────────────┘

5) NETWORK TAB
   ┌─────────────────────────────────────────────────────────────────────────────────────────┐
   │ Mode: [OFF/LAN Host/LAN Client/Follow]                                                 │
   │ Multicast/Broadcast: IP [224.25.26.27] Port [4665]                                     │
   │ TCP settings, Receive from host                                                        │
   │ Follow system type mapping                                                             │
   └─────────────────────────────────────────────────────────────────────────────────────────┘

6) HARDWARE / GPIO / SERIAL TAB
   ┌─────────────────────────────────────────────────────────────────────────────────────────┐
   │ Serial: [COM/ttyUSB] Port1/Port2                                                       │
   │ Arduino: [ON/OFF]                                                                       │
   │ K8055: [ON/OFF] Button mapping                                                         │
   │ RPi GPIO / Input mapping                                                               │
   │ Keyboard/Remote controls mapping                                                       │
   │ Laser pointers                                                                         │
   └─────────────────────────────────────────────────────────────────────────────────────────┘

7) EMERGENCY TAB
   ┌─────────────────────────────────────────────────────────────────────────────────────────┐
   │ Behavior: [Stop current / Stop next / Pause]                                           │
   │ Emergency text/sound                                                                   │
   └─────────────────────────────────────────────────────────────────────────────────────────┘

8) LANGUAGE TAB
   ┌─────────────────────────────────────────────────────────────────────────────────────────┐
   │ Language: [English/Nederlands/Deutsch/Francais/...]                                    │
   └─────────────────────────────────────────────────────────────────────────────────────────┘

9) STREAMING TAB (ffmpeg)
   ┌─────────────────────────────────────────────────────────────────────────────────────────┐
   │ Enable: [ON/OFF]                                                                       │
   │ Output: [RTMP URL / UDP / File]                                                        │
   │ Resolution/FPS/Bitrate/Codec (libx264)                                                 │
   │ [Start] [Stop] [Status]                                                                │
   └─────────────────────────────────────────────────────────────────────────────────────────┘

FORMAT-SPECIFIC OPTIONS (shown contextually)
- FITA: System type (1x1/2x1 etc), ABCD per end, alternation, times per end, SO times
- FITA Finals: Arrow/Target mode, choose archer, final timing
- 25m 1+1P: Shooters, rotation, per-arrow times, SO
- Manual: Full manual control
- Follow: Client of remote host
```
