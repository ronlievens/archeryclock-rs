# UC-012: Display Customization

## User Story
**As an** Operator  
**I want** to customize display (colors, layout, banners, fullscreen, language)  
**So that** it's visible in different venues

## Preconditions
- Application running

## Acceptance Criteria
```gherkin
Feature: Display customization

  Scenario: Change color scheme
    Given color schemes available
    When operator selects scheme (1-9) or custom colors
    Then colors update for all states

  Scenario: Configure per-state colors
    Given custom colors enabled
    When operator sets colors for green/orange/red/background/etc.
    Then applied immediately

  Scenario: Banners and backgrounds
    Given banner images/slides configured
    When enabled
    Then banners/backgrounds shown

  Scenario: Fullscreen mode
    Given fullscreen setting
    When enabled
    Then display goes fullscreen

  Scenario: Layout options
    Given left/right view, split display
    When configured
    Then layout adjusts

  Scenario: Language selection
    Given language files exist
    When operator selects language
    Then all labels updated

  Scenario: Show/hide elements
    Given options like hide icons, show hints, show end nr
    When toggled
    Then elements shown/hidden
```

## Technical Notes
- Color schemes via `changecolorscheme()`
- `bannerpicture`, `defaultbannerRadioButton`, `specificbannerRadioButton`
- `fullscreencheckbox`
- `Lang` combo, `setlanguage()`
- `hideicons`, `showhints`

## GUI Wireframe (Colors/Layout/Display)

```text
Colors tab: Schemes 1-9, Per-state colors, Banner [Default/Custom][Browse],
Fullscreen [✓], Hide icons [✓], Show hints [✓], Language selector
```
