# BDD Implementation Guide

## Purpose
Use the split BDD use cases in `docs/usecases/` to generate code in Rust + SDL2.

## Mapping to modules (suggested)
- `src/main.rs` / `src/app.rs` - Application lifecycle (UC-001)
- `src/config/` - Settings/scenarios loading/saving (UC-001, UC-017)
- `src/format/` - Format types and logic (UC-002..UC-006)
- `src/state/` - State machine (UC-008, UC-009)
- `src/timer/` - Countdown timing
- `src/audio/` - Audio/buzzer (UC-011)
- `src/display/` - Rendering/SDL2 UI (UC-012)
- `src/network/` - UDP/TCP/follow (UC-007, UC-013)
- `src/hw/` - Serial/Arduino/K8055/GPIO (UC-014)
- `src/ap/` - Access point (UC-015, Linux)
- `src/scenario/` - Scenarios F1-F12 (UC-010)
- `src/stream/` - FFmpeg streaming (UC-016)

## Implementation order
1. Config + startup (UC-001, UC-017)
2. State machine + timer (UC-008)
3. Format selection + basic FITA (UC-002, UC-003)
4. Display rendering (UC-012)
5. Audio (UC-011)
6. Emergency (UC-009)
7. Scenarios (UC-010)
8. 25m1P/Finals/Manual/Follow (UC-004, UC-005, UC-006, UC-007)
9. Network (UC-013)
10. Hardware I/O (UC-014)
11. AP (UC-015)
12. FFmpeg streaming (UC-016)

## SDL2 requirements
- Use `sdl2` crate with features: `ttf`, `image`, `mixer`
- Cross-platform Windows/Linux/macOS
- Render clock display prominently, menus/tabs for configuration

## FFmpeg streaming details
- Spawn ffmpeg as subprocess
- Pipe raw frames from SDL surface (e.g. RGB24/BGR24)
- Configurable: input format, resolution, fps, codec, bitrate, output URL/target
- Handle process lifecycle

## Testing
- Unit tests for state machine and timing
- Integration tests for config parsing
- Manual testing per format

## References
- See `docs/usecases/` for detailed acceptance criteria
- See `wireframes/MAIN_GUI_ASCII.txt` for UI layout
- See `orginal-src/Countdown.pas` for behavior details
- See `orginal-exe/` for actual assets and example configs/scenarios
