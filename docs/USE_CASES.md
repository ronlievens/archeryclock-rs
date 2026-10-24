# ArcheryClock Use Cases (from orginal-src analysis)

This document describes use cases extracted from the original ArcheryClock (Lazarus/Pascal) codebase. It is intended as the basis for reimplementation in Rust.

## References (official / canonical)
- World Archery / FITA rules context: https://www.worldarchery.sport/rulebook
- World Archery Rules Book (current): https://extranet.worldarchery.sport/documents/index.php/Rules/
- FITA/WA competition formats (common): Recurve/Compound 70m, 50m, 18m, etc. Timing varies by format; rules define ends, arrows per end, time per end/turn.
- 25m 1+1P (Precision and Rapid) / 25m women/men formats - timing differs (e.g. 5 ends of 5 arrows in Precision + Rapid in some forms, or per-arrow/series as modeled). See World Archery rulebook for current 25m formats.
- ArcheryClock project site / original info: http://www.archeryclock.com/ (mentioned in source comments)

Notes from source: "Archery countdown system" (README). Copyright 2010-2016 Henk Jegers. Uses SDL, SDPO serial, LNET, MMSystem (Windows). Linux contributions by Yann Morere.

## System overview
- Archery countdown/timer for competition use with multiple display/operating modes.
- Supports different archery systems/formats: `FITA` (outdoor/indoor FITA style), `fitafin` (finals/shoot-off style), `25m1p` (25m 1+1 program), `manual` (manual control), `follow` (client/follow mode - receives state from host).
- Networked: LAN UDP/multicast/broadcast and TCP (lNet components). Also serial/RS232, GPIO/parallel/K8055 (Windows), Arduino, Raspberry Pi inputs, access point mode (Linux).
- Audio: WAV sounds, volume control, buzzer/MP3/VLC support, remote audio control.
- Scenarios: programmable sequences (F1-F12 / custom) stored in `scenarios/` files.
- Visual: multiple color schemes, banners, backgrounds, full screen, left/right split views, FITA finals arrow/target views.


## Actors
- **Timekeeper/Operator**: runs the clock, controls start/next/stop/emergency via keyboard/GUI/buttons/serial/remote/network.
- **Archers**: shoot according to signals/timer.
- **Judge/officials**: may use finals controls (choose archer, etc.)
- **Spectators/viewers**: see large display (clock/colors/arrows/targets/banners).
- **Remote clients**: follow/receive state over network (follow mode).
- **External hardware**: serial devices, Arduino, K8055 (USB I/O), GPIO/RPi, network remotes.

## Core concepts / states
From code (`state` variable): states include 1 (wait/pre/ready), 2 (green/shoot), 3 (orange/warning), 4 (red/last seconds), 5 (countdown to start), 6 (emergency). 
Also `realstate`, `prevstate`. Emergency mode has configurable behavior.

### Archery systems (`archerysystem`)
- `fita` - standard FITA/WA format with ends, turns/series, configurable per-end times, ABCD archer sequences, alternation rules.
- `fitafin` - finals/shoot-off mode (arrows/target style, choose archer, different timing/display).
- `25m1p` - 25m 1+1 precision/rapid style (per-arrow timing shown differently, archer rotation concepts).
- `manual` - manual timing control (operator controls phases manually).
- `follow` - follows another clock via network (`followsystem` indicates source type).

### Timing modes
- Time per end (`timeperend`) vs time per turn (`timeperturn`) in finals contexts.
- Minutes/seconds display (`minsec`/`seconds`), non-zero seconds display option.
- Different times for practice vs competition (`practiseturns`), shoot-off times.
- FITA finals can use arrow-based or target-based timing (`finarrow`, `fintarget`).
- 25m1P uses per-arrow times with shoot-off variants.

## Use cases

### UC-01: Start application and configure
**Actor:** Operator  
**Preconditions:** Software installed with assets (sounds, images, language, scenarios).  
**Flow:**
1. Launch application.
2. Load settings from `settings` file (language, display options, network/serial/hardware, audio, scenario defaults).
3. Load language file.
4. Initialize display, timers, I/O (serial/network/hardware) based on settings.
5. Apply startup scenario if configured.
6. Show main clock/display screen or menu depending on config.
**Postconditions:** Clock ready in configured mode.

### UC-02: Select competition format/system
**Actor:** Operator  
**Flow:**
1. Choose archery system: FITA, FITA Finals, 25m 1+1P, Manual, Follow.
2. System applies format-specific defaults (times, UI elements, sequences).
3. Configure format details (number of archers/shooters, ABCD sequences, rotation, shoot-off settings).
**Notes:** Switching affects display labels, allowed operations, timing logic.

### UC-03: Configure timing per phase (FITA)
**Actor:** Operator  
**Flow:**
1. Set preparation/warning/shoot times per end/turn (green/orange/red).
2. Set shoot-off times separately.
3. Configure practice turns count.
4. Set behavior for last seconds, beeps.
**References:** World Archery timing rules for relevant format.

### UC-04: Configure 25m 1+1P
**Actor:** Operator  
**Flow:**
1. Set times for 1+1P (precision/rapid as applicable).
2. Configure number of shooters/arrows, archer rotation (`rotate25m1p`).
3. Set shoot-off times.
4. Handle per-arrow display/numbering.

### UC-05: Configure FITA Finals / shoot-off
**Actor:** Operator  
**Flow:**
1. Select arrow-based or target-based timing.
2. Configure times per arrow/target, shoot-off variants.
3. Choose starting archer/side.
4. Manage final sequence.

### UC-06: Run countdown sequence (normal)
**Actor:** Operator/Timekeeper  
**Flow:**
1. Operator triggers start/next (keyboard, GUI, hardware button, network command).
2. System transitions through states: prepare/wait -> green (shoot) -> orange (warning) -> red (stop/last).
3. Audio signals/beeps at configured points.
4. Display updates (colors, digits, archer labels, end/turn info).
5. On phase end, system may auto-advance or wait for operator depending on config.
6. Track ends/turns/series according to scenario/format.

### UC-07: Emergency stop/pause
**Actor:** Operator/Judge  
**Flow:**
1. Trigger emergency (button/command).
2. System goes to emergency state (configurable: stop current, stop next, pause).
3. Display emergency indication/text.
4. Operator can resume or reset.

### UC-08: Program and run scenarios (F1-F12)
**Actor:** Operator  
**Flow:**
1. Define scenario sequence (timing, phases, repeats) - stored in scenario files.
2. Load scenario (F1-F12 or custom).
3. Run scenario automatically or stepwise.
4. Scenarios can be saved/loaded.

### UC-09: Network synchronization (follow mode / host-client)
**Actor:** Operator/System  
**Flow:**
1. Host broadcasts/sends state over UDP multicast/broadcast/TCP.
2. Clients in follow mode receive and mirror display/state.
3. Supports different follow system types mapping to formats.
4. Can sync time/settings as implemented.

### UC-10: Remote control / external inputs
**Actor:** Operator  
**Flow:**
1. Control via keyboard shortcuts, on-screen buttons, serial (RS232), Arduino, K8055, GPIO (RPi), network, laser pointers/remote controls.
2. Map inputs to actions (start/next/stop/emergency/scene change/etc.).

### UC-11: Audio and signaling
**Actor:** System  
**Flow:**
1. Play WAV sounds for phase changes/beeps.
2. Control volume, buzzer (hardware), remote audio (VLC/MP3/network audio control).
3. Configurable signals per state/phase.

### UC-12: Display customization
**Actor:** Operator  
**Flow:**
1. Configure colors per phase, color schemes.
2. Show/hide elements (archer names, end numbers, hints).
3. Fullscreen, screen layout (left/right), banners/background images.
4. Language selection.

### UC-13: Access point / network setup (Linux/RPi)
**Actor:** Operator  
**Flow:**
1. Configure WiFi AP (hostapd/dhcpcd) via UI.
2. Set SSID/password/channel.
3. Enable/disable AP.


## Detailed functional requirements for reimplementation (Rust + SDL)
Based on original behavior and `requirements.md`:

### Must have
- Multi-format support: FITA, FITA Finals (shoot-off), 25m 1+1P, Manual, Follow (client).
- State machine: wait/ready, shoot (green), warning (orange), last/red, countdown-to-start, emergency.
- Configurable timing per format/phase (green/orange/red, shoot-off, practice).
- End/turn/series tracking, archer/shooter management and rotation rules as in original.
- Manual/operator control (keyboard + UI) to advance phases.
- Audio beeps/signals.
- Network sync (follow/host) - at least UDP multicast/broadcast compatible with original protocol if possible.
- Serial/hardware input support? (may be optional for first cross-platform SDL version, but design should allow). Original supports many inputs.
- Scenarios (F1-F12) loading/running.
- Display with colors, time remaining, archer indicators, end numbers.
- Cross-platform: Windows, Linux, macOS - all on SDL (as requested). Use SDL2/SDL3 via Rust bindings (e.g. `sdl2` crate). 
- Streaming option using ffmpeg (as requested).
- Configuration persistence.

### Should have
- Multiple languages.
- Color schemes, banners/backgrounds.
- Fullscreen support.
- Finals modes (arrow/target), choose archer.
- Shoot-off handling.
- Remote audio control.

### Nice to have
- Access point config (Linux-specific) - maybe defer or make pluggable.
- GPIO/K8055/Arduino specific - may need abstraction; keep interfaces.

## Architecture notes for Rust reimplementation
- **UI/Rendering**: SDL2 (or SDL3). For cross-platform, SDL is good. Consider `sdl2` crate with image/ttf/mixer.
- **State machine**: central state (format, phase, times, ends/turns, archers).
- **Config**: TOML/JSON/INI (original uses custom settings file; can support compatible format or new).
- **Assets**: sounds (WAV), images (PNG/JPG), fonts, languages.
- **Networking**: `std::net` UDP/TCP, tokio optional or std.
- **Serial**: `serialport` crate (cross-platform).
- **FFmpeg streaming**: use `ffmpeg` CLI via subprocess or `ffmpeg-sys-next`/`rust-ffmpeg` bindings. Requirement says "gebruik ffmpeg voor de streaming optie" - can spawn ffmpeg to stream display/capture.
- **Time**: monotonic timer for countdown (independent of wall clock as per requirements).

## ASCII wireframe (current GUI concept)
Based on original UI layout (tabs: Scenario, Pre-count, Screen colors, Sound, Network, Hardware/GPIO, Emergency, Language, About; main clock display with left/right sides, archers, end numbers, colors). See attached wireframe below.


## Archery Clock options - detailed description (with references)
This section details options found in original code/settings for clarity.

### General/Display
- **Language**: multi-language support (files in `language/`).
- **Fullscreen** (`nonfullscreen`/`fullscreen`): toggle fullscreen display.
- **Hide icons/menu**: cleaner display for spectators.
- **Show hints**: helper text on screen.
- **Banners/backgrounds**: custom images, slides/commercial mode.
- **Color schemes**: 9 schemes + per-state custom colors.
- **Screen layout**: left/right view, split display options.
- **Time format**: minutes:seconds or seconds only; hide leading zero option.
- **Date/time display** on screen in certain modes.

### Competition formats
- **FITA**: standard WA/FITA format. Configurable archer groups (1x1 up to larger teams), ends/turns, alternation (`ABCDperEND`), times per end.
- **FITA Finals (fitafin)**: shoot-off/finals. Modes: arrow-based or target-based. Can select archer (left/right/choose). Different timing.
- **25m 1+1P**: 25 meter Precision and Rapid. Per-arrow timing, archer rotation (`rotate25m1p`), different labels. Shoot-off times.
- **Manual**: operator manually controls all phases.
- **Follow**: network client mirroring host clock (maps to different system types).

References: World Archery rules - timing and format details vary by discipline/event; see https://www.worldarchery.sport/rulebook

### Timing configuration
- **Time per end vs time per turn**: finals can differ.
- **Green/Orange/Red times**: configurable per format (and shoot-off).
- **Practice turns**: different timing/count for practice.
- **Shoot-off times**: separate green/orange/red for shoot-offs.
- **Pre-count**: countdown to start of match/end.
- **Last seconds behavior / beeps**: configurable.

### Archer/sequence management
- **Shooters count**: 1-6+ depending on mode.
- **ABCD sequences**: order of shooting per end/turn.
- **Rotation**: 25m1P has rotation options.
- **Finals archer selection**: choose which archer shoots.
- **Alternation**: different alternation patterns.

### Audio
- **Sounds per state**: assign WAV files to phases.
- **Buzzer/hardware buzzer**: external buzzer support.
- **Volume control**: per-state/master.
- **Remote audio**: VLC/MP3/network control, TCP audio commands.
- **Beeps**: configurable beep patterns/counts.

### Control inputs
- **Keyboard**: extensive shortcuts (F-keys, arrows, etc.).
- **Serial/RS232**: external control devices.
- **Arduino**: custom Arduino integration.
- **K8055**: Velleman K8055 USB interface (Windows).
- **GPIO/Raspberry Pi**: GPIO inputs.
- **Network/remote**: TCP/UDP control.
- **Laser pointers / presentation remotes**: mapped inputs.
- **Touchscreen** support.

### Networking
- **Host/Client**: send/receive state.
- **Multicast/Broadcast**: configurable address/port (default 224.25.26.27:4665 in settings).
- **Follow mode**: client follows host with format mapping.
- **Remote shutdown/control**.

### Hardware/IO details (Linux/RPi focus in parts)
- **Access Point**: configure hostapd for WiFi AP (SSID/password/channel/country).
- **Serial ports**: /dev/tty* or COM*.

### Scenarios
- **F1-F12 programmable**: store complete timing/sequences.
- **Startup scenario**: auto-load on start.

## Streaming option (ffmpeg)
**Requirement:** "gebruik ffmpeg voor de streaming optie"  
Implementation approach:
- Capture SDL window/display or render surface.
- Pipe frames to ffmpeg via stdin (rawvideo) or use screen capture.
- Support common outputs: RTMP (e.g. to streaming platforms), UDP, file (HLS/MP4), or local streaming.
- Configurable resolution, framerate, bitrate, encoder (libx264).
- Cross-platform: ffmpeg must be available on system or bundled.
- UI panel for streaming settings (enable/disable, URL, options).
- Start/stop streaming tied to app or manual control.

## Implementation plan for Rust
1. **Project setup**: Add dependencies (sdl2, sdl2-image, sdl2-ttf, sdl2-mixer, serde, toml, serialport, etc.). Make cross-platform.
2. **Port core logic**: State machine, timing, formats (FITA/25m1P/Finals/Manual/Follow), scenarios.
3. **Rendering**: SDL2 UI with clock display, menus/tabs (or simpler UI first), ASCII wireframe as guide.
4. **Config**: Load/save settings (TOML) with migration path from original format if desired.
5. **Audio**: SDL2_mixer for WAV/beeps.
6. **Networking**: UDP/TCP for follow/host and control.
7. **Serial/IO**: Abstract inputs (keyboard first, then serial/network).
8. **FFmpeg streaming**: Module to spawn ffmpeg with appropriate args, pipe frames.
9. **Testing**: Basic timing tests.
10. **Docs**: Keep use cases updated.
