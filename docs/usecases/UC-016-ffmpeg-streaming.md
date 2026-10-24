# UC-016: FFmpeg Streaming Option

## User Story
**As an** Operator  
**I want** to stream the clock display using ffmpeg  
**So that** spectators can watch remotely

## Preconditions
- ffmpeg installed and available in PATH
- Display rendering available (SDL surface/window)

## Acceptance Criteria
```gherkin
Feature: FFmpeg streaming

  Scenario: Configure streaming
    Given streaming settings panel
    When operator enables streaming, sets output URL/target (RTMP/UDP/file) and options (resolution, bitrate, fps)
    Then configuration saved

  Scenario: Start streaming
    Given configured and ffmpeg available
    When operator starts streaming
    Then ffmpeg process spawned with appropriate args
    And SDL frames captured/piped to ffmpeg (rawvideo or via suitable method)
    And streaming active indicator shown

  Scenario: Stop streaming
    Given streaming active
    When operator stops streaming
    Then ffmpeg process terminated cleanly
    And streaming stopped

  Scenario: Cross-platform support
    Given Windows/Linux/macOS
    When streaming
    Then works on all platforms with ffmpeg installed

  Scenario: Error handling
    Given ffmpeg not found or bad config
    When starting
    Then error shown to operator
```

## Technical Notes
- Requirement: "gebruik ffmpeg voor de streaming optie"
- Options: pipe raw RGB frames from SDL texture/surface to ffmpeg stdin, or use platform capture
- Common ffmpeg args: `-f rawvideo -pix_fmt rgb24 -s WxH -r fps -i - -c:v libx264 ... output`
- Or use `x11grab`/`gdigrab`/`avfoundation` depending on platform, but piping from app is cleaner and cross-platform
- Need to add streaming UI panel (as per wireframe)
- Dependencies: spawn process, handle stdin

## GUI Wireframe (Streaming)

```text
Streaming tab: Enable [✓], Output [RTMP/UDP/File ▼], URL/Path, Resolution [1280x720], FPS [30], Bitrate [2000k], Codec [libx264],
[Start Streaming] [Stop Streaming] Status: [Idle/Streaming]
```
