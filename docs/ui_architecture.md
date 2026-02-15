# UI Architecture

## Overview

Nodoka's user interface is built using the [iced](https://github.com/iced-rs/iced) framework (version 0.12), which implements the Elm architecture pattern. This document explains the UI architecture, design patterns, and implementation details.

## Elm Architecture Pattern

The UI follows the Elm architecture with three core concepts:

### Model (State)

The `State` struct (`src/ui/state.rs`) represents the complete UI state:

```rust
pub struct State {
    // Data
    pub audiobooks: Vec<Audiobook>,
    pub selected_audiobook: Option<i64>,
    pub current_files: Vec<AudiobookFile>,
    pub selected_file: Option<String>,
    pub bookmarks: Vec<Bookmark>,
    pub directories: Vec<Directory>,
    
    // Playback state
    pub is_playing: bool,
    pub current_time: f64,
    pub total_duration: f64,
    pub volume: i32,
    pub speed: f32,
    
    // UI state
    pub settings_open: bool,
    pub bookmark_editor: Option<BookmarkEditor>,
    pub error_message: Option<String>,
    pub is_scanning: bool,
    pub focused_element: FocusedElement,
}
```

The state is:
- **Immutable from UI perspective**: UI cannot directly modify state
- **Single source of truth**: All UI rendering derives from state
- **Serializable**: Can be saved/restored (not currently implemented)

### Update (Message Handlers)

Messages (`src/ui/message.rs`) represent all possible user interactions:

```rust
pub enum Message {
    // Playback
    PlayPause,
    Stop,
    SeekForward(i32),
    SeekBackward(i32),
    
    // Selection
    AudiobookSelected(i64),
    FileSelected(String),
    
    // Keyboard shortcuts
    KeyPressed(shortcuts::ShortcutKey, iced::keyboard::Modifiers),
    
    // Modal interactions
    OpenSettings,
    CloseSettings,
    EscapePressed,
}
```

The `update` module (`src/ui/update/`) handles messages and returns:
1. Updated state
2. `Command<Message>` for async operations

```rust
pub fn update(state: &mut State, message: Message) -> Command<Message> {
    match message {
        Message::PlayPause => {
            state.is_playing = !state.is_playing;
            Command::none()
        }
        Message::AudiobookSelected(id) => {
            state.selected_audiobook = Some(id);
            // Return command to load files asynchronously
            Command::perform(
                load_files_async(id),
                Message::FilesLoaded
            )
        }
    }
}
```

### View (Rendering)

The view function (`src/ui/main_window.rs`) renders state to UI elements:

```rust
pub fn view(state: &State) -> Element<Message> {
    // Render UI based on current state
    // No business logic - pure function of state
    column![
        player_controls::view(state),
        audiobook_list::view(&state.audiobooks, state.selected_audiobook),
        file_list::view(&state.current_files, state.selected_file),
    ]
    .into()
}
```

Views are:
- **Pure functions**: Same state always produces same UI
- **Declarative**: Describe what to render, not how
- **Composable**: Built from smaller component views

## Component Hierarchy

```
main_window::view()
├── player_controls::view()
│   ├── Play/Pause button
│   ├── Stop button
│   ├── Volume slider
│   ├── Speed slider
│   ├── Speed presets
│   └── Sleep timer
├── audiobook_list::view()
│   └── List of audiobooks with covers
├── file_list::view()
│   └── List of chapter files
├── bookmarks::view()
│   ├── Bookmark list
│   └── Bookmark editor (modal)
└── settings_form::view() (modal)
    └── Directory management
```

## Message Flow

```
User clicks Play button
        ↓
[View generates Message::PlayPause]
        ↓
[update() receives message and state]
        ↓
[State.is_playing toggles]
        ↓
[Command returned (possibly Command::none())]
        ↓
[iced re-renders with new state]
        ↓
[View shows paused UI]
```

## Async Operations (Commands)

Commands handle asynchronous operations:

```rust
// Scan directory (blocking I/O)
Command::perform(
    async move {
        scan_directory_blocking(path)
    },
    |result| Message::DirectoryScanComplete(result)
)

// Update database (blocking I/O)
Command::perform(
    async move {
        database.save_progress(file_path, position)
    },
    |result| Message::ProgressSaved(result)
)
```

Commands:
- Run on background threads
- Generate new messages when complete
- Never block the UI thread

## State Management Patterns

### Modal Management

Modals are controlled via state fields:

```rust
// Open modal
state.settings_open = true;

// Close modal (Escape key priority)
if state.settings_open {
    state.settings_open = false;
} else if state.bookmark_editor.is_some() {
    state.bookmark_editor = None;
}
```

Priority: Settings modal > Bookmark editor

### Error Handling

Errors are displayed via error banner:

```rust
state.error_message = Some("Failed to load file".to_string());
state.error_timestamp = Some(chrono::Utc::now());

// Auto-dismiss after 5 seconds (handled by update logic)
```

### Loading States

Long operations show loading indicators:

```rust
state.is_scanning = true;
state.scanning_directory = Some(path.to_string());

// UI shows: "Scanning: /path/to/audiobooks..."

// On completion:
state.is_scanning = false;
state.scanning_directory = None;
```

## Keyboard Navigation

Keyboard shortcuts are centralized in `shortcuts.rs`:

```rust
pub enum ShortcutKey {
    Space,
    LeftArrow,
    RightArrow,
    UpArrow,
    DownArrow,
    B,  // Ctrl/Cmd+B for bookmarks
    Escape,
}

pub fn message_for_key_chord(
    key: ShortcutKey,
    modifiers: Modifiers,
) -> Option<Message> {
    match (key, modifiers) {
        (ShortcutKey::Space, _) => Some(Message::PlayPause),
        (ShortcutKey::LeftArrow, _) => Some(Message::SeekBackward(5)),
        // ...
    }
}
```

Key handling in `app.rs`:

```rust
fn map_key_press(
    key: iced::keyboard::Key,
    modifiers: iced::keyboard::Modifiers,
) -> Option<Message> {
    let shortcut_key = match key {
        iced::keyboard::Key::Named(iced::keyboard::key::Named::Space) => {
            ShortcutKey::Space
        }
        // Map iced keys to ShortcutKey enum
    };
    
    shortcuts::message_for_key_chord(shortcut_key, modifiers)
}
```

Focus tracking (accessibility):

```rust
pub enum FocusedElement {
    None,
    PlayPauseButton,
    VolumeSlider,
    AudiobookList,
    FileList,
    // ...
}

state.focused_element = FocusedElement::PlayPauseButton;
```

Note: iced 0.12 does not expose focus state directly, so we track it manually in application state.

## Styling System

Styles are defined in `styles.rs`:

```rust
pub mod colors {
    pub const PRIMARY: Color = Color::from_rgb(0.8, 0.2, 0.4);  // Rose
    pub const BG_PRIMARY: Color = Color::from_rgb(0.1, 0.1, 0.1);
    pub const TEXT_PRIMARY: Color = Color::from_rgb(0.95, 0.95, 0.95);
}

pub mod spacing {
    pub const XS: f32 = 4.0;
    pub const SM: f32 = 8.0;
    pub const MD: f32 = 16.0;
    // All spacing follows 4px grid
}
```

### Button Styling Workaround

**Limitation**: iced 0.12 does not support custom button styles via `.style()` method.

**Workaround**: Wrap buttons in styled containers:

```rust
// Instead of:
button("Play").style(primary_button_style)  // NOT SUPPORTED

// Use:
container(
    button("Play").on_press(Message::PlayPause)
)
.style(button_containers::primary())
.padding(spacing::SM)
```

Container styles in `styles.rs`:

```rust
pub mod button_containers {
    pub fn primary() -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(colors::PRIMARY)),
            border: Border {
                color: colors::PRIMARY,
                width: 2.0,
                radius: border_radius::MD.into(),
            },
            ..Default::default()
        }
    }
    
    pub fn secondary() -> container::Appearance { /* ... */ }
    pub fn danger() -> container::Appearance { /* ... */ }
}
```

Visual hierarchy:
- **Primary**: Vibrant rose background (Play, Save, Add)
- **Secondary**: Elevated background with border (Cancel, Close)
- **Danger**: Error color (Delete, Remove)

See `docs/ui_limitations.md` for detailed workaround patterns.

## Focus Indicators

WCAG 2.1 AA requires visible focus indicators (≥3px).

Implementation:

```rust
pub fn focus_indicator(is_focused: bool) -> container::Appearance {
    if is_focused {
        container::Appearance {
            border: Border {
                color: colors::FOCUS_RING,  // Blue #2563EB
                width: 3.0,
                radius: border_radius::MD.into(),
            },
            ..Default::default()
        }
    } else {
        container::Appearance::default()
    }
}
```

Usage:

```rust
container(button("Play"))
    .style(if state.focused_element == FocusedElement::PlayPauseButton {
        focus_indicator(true)
    } else {
        button_containers::primary()
    })
```

## iced 0.12 Limitations

See `docs/ui_limitations.md` for comprehensive list. Key limitations:

1. **No custom button styles**: Use container workaround
2. **No focus state exposure**: Track in application state
3. **No modal backdrop**: Use Escape key and explicit close buttons
4. **Limited animation support**: Minimal transitions

Upgrade path: iced 0.13+ will support custom button styles and better focus management.

## Database Integration

UI does not directly access database. Instead:

1. Messages trigger commands
2. Commands call `db::queries` functions
3. Results generate new messages
4. Update function modifies state

```rust
// User clicks "Add Directory"
Message::DirectoryAdd(path) => {
    Command::perform(
        async move {
            let db = get_db_connection();
            db::queries::insert_directory(&db, &Directory::new(path))
        },
        |result| Message::DirectoryAdded(result)
    )
}

// Result arrives
Message::DirectoryAdded(Ok(())) => {
    state.directories = load_directories();  // Refresh list
    Command::none()
}

Message::DirectoryAdded(Err(e)) => {
    state.error_message = Some(format!("Failed: {}", e));
    Command::none()
}
```

## Player Integration

VLC player is controlled via `player` module (not UI):

```rust
// User clicks Play
Message::PlayPause => {
    if state.is_playing {
        player.pause();
    } else {
        player.play();
    }
    state.is_playing = !state.is_playing;
    Command::none()
}
```

Player communicates back via messages:

```rust
// Player sends position updates
Message::TimeUpdate(time) => {
    state.current_time = time;
    Command::none()
}

// Player sends end-of-file notification
Message::FileEnded => {
    // Advance to next file
    if let Some(next) = get_next_file(&state.current_files, &state.selected_file) {
        Command::perform(
            async { next },
            Message::FileSelected
        )
    } else {
        state.is_playing = false;
        Command::none()
    }
}
```

## Testing Strategy

See `docs/testing_strategy.md` for comprehensive testing approach.

### Unit Tests

Component tests verify rendering:

```rust
#[test]
fn test_player_controls_render() {
    let state = State::default();
    let view = player_controls::view(&state);
    // View rendering doesn't panic
}
```

### Integration Tests

State transition tests:

```rust
#[test]
fn test_play_pause_workflow() {
    let mut state = State::default();
    
    // Start playing
    update(&mut state, Message::PlayPause);
    assert!(state.is_playing);
    
    // Pause
    update(&mut state, Message::PlayPause);
    assert!(!state.is_playing);
}
```

### Manual Testing

Visual verification required for:
- Button visual hierarchy
- Focus indicators
- Modal backdrops
- Loading states
- Error banners

See `tests/manual_ui_checklist.md`.

## Performance Considerations

### Rendering Optimization

- Use `Vec::with_capacity()` for large lists
- Avoid allocations in hot paths (view functions)
- Delegate heavy work to commands (background threads)

### Large Datasets

For 100+ audiobooks or 1000+ files:
- Virtualization not available in iced 0.12
- Consider pagination or filtering in future
- Current implementation tested up to 200 audiobooks (see `tests/ui_performance_tests.rs`)

### State Size

State size is reasonable (<1MB typically):
- Audiobook list: ~100 items × ~1KB = 100KB
- File list: ~1000 items × ~500B = 500KB
- Bookmarks: ~1000 items × ~200B = 200KB

Cloning is cheap due to reference counting (via database IDs).

## Accessibility

Nodoka follows WCAG 2.1 Level AA:

- **2.1.1 Keyboard**: All functionality via keyboard
- **2.4.7 Focus Visible**: 3px blue focus ring
- **1.4.3 Contrast**: Text contrast ratio ≥ 4.5:1
- **3.3.1 Error Identification**: Clear error messages
- **4.1.3 Status Messages**: Loading states announced

See `tests/accessibility_tests.rs` for automated checks.

Manual screen reader testing required:
- macOS: VoiceOver
- Windows: NVDA
- Linux: Orca

## Future Enhancements

When upgrading to iced 0.13+:
1. Replace button container workaround with native button styles
2. Use native focus tracking instead of manual state
3. Add modal backdrop support
4. Implement view virtualization for large lists
5. Add smooth animations/transitions

## References

- [iced Documentation](https://docs.rs/iced/)
- [Elm Architecture](https://guide.elm-lang.org/architecture/)
- [WCAG 2.1](https://www.w3.org/WAI/WCAG21/quickref/)
- Design system: `design-system/nodoka-audiobook-player/MASTER.md`
- Limitations: `docs/ui_limitations.md`
- Testing: `docs/testing_strategy.md`
