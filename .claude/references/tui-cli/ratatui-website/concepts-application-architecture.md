# Application Architecture

<!-- source: https://ratatui.rs/concepts/application-patterns/the-elm-architecture/ -->
<!-- source: https://ratatui.rs/tutorials/counter-app/basic-app/ -->
<!-- source: https://ratatui.rs/templates/component/app-rs/ -->
<!-- fetched: 2026-03-15 (via WebSearch) -->

## Core Philosophy

Ratatui is based on the principle of **immediate rendering with intermediate buffers**. Each
frame, the application must render all widgets that should be part of the UI. This is in
contrast to retained-mode rendering where widgets are updated and automatically redrawn.

The user handles the event loop, the application state, and redraws the entire UI on each
iteration. Ratatui does not handle input; use a backend library like crossterm for that.

## Basic Event Loop

A minimal application:

```rust
fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::default().run(terminal);
    ratatui::restore();
    result
}

impl App {
    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => self.running = false,
                _ => {}
            }
        }
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}
```

Always use a **separate function** for the main loop. This ensures `ratatui::restore()` is
called even if an error causes an early return via `?`.

## App Struct Pattern

All business logic lives in an `App` struct. State is stored as fields. Updates happen via
methods rather than inline in the match statement — this enables unit testing state transitions
independently of event handling.

```rust
#[derive(Default)]
struct App {
    running: bool,
    counter: u32,
}

impl App {
    fn increment(&mut self) {
        self.counter = self.counter.saturating_add(1);
    }

    fn decrement(&mut self) {
        self.counter = self.counter.saturating_sub(1);
    }
}
```

Implement `Widget for &App` so the app can render itself without being consumed:

```rust
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ]);
        let [header, content, footer] = area.layout(&layout);
        self.render_header(header, buf);
        self.render_content(content, buf);
        self.render_footer(footer, buf);
    }
}
```

## Event Handling

Event handling is performed by the backend (crossterm), not Ratatui. Three common strategies:

### Blocking Reads (Simple — Recommended for most apps)

```rust
fn handle_events(&mut self) -> Result<()> {
    let timeout = Duration::from_millis(50);
    if event::poll(timeout)? {
        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Char('q') => self.quit(),
                KeyCode::Up => self.prev(),
                KeyCode::Down => self.next(),
                _ => {}
            }
        }
    }
    Ok(())
}
```

`event::poll(timeout)` returns without blocking if no event arrives within the timeout,
enabling regular frame redraws (e.g., for animations or periodic updates).

### Channel-Based (Async-friendly)

Use `mpsc` channels to decouple event reading from rendering:

```rust
// Spawn a dedicated thread for reading events
let (tx, rx) = std::sync::mpsc::channel();
std::thread::spawn(move || {
    loop {
        if let Ok(event) = crossterm::event::read() {
            tx.send(event).ok();
        }
    }
});

// In the main loop, non-blocking receive:
while let Ok(event) = rx.try_recv() {
    // handle event
}
```

### Async with Tokio

For I/O-intensive apps, use `crossterm::event::EventStream` with tokio:

```rust
use crossterm::event::EventStream;
use tokio_stream::StreamExt;

async fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut events = EventStream::new();
    loop {
        tokio::select! {
            _ = tokio::time::sleep(Duration::from_millis(16)) => {
                terminal.draw(|f| render(f))?;
            }
            Some(Ok(event)) = events.next() => {
                handle(event);
            }
        }
    }
}
```

## The Elm Architecture (TEA)

TEA provides a proven structure for larger applications:

**Model**: The application's state — all data the app works with.

**Message**: An enum representing all possible state-changing events.

**Update**: A function `(Model, Message) -> Model`. Produces a new model rather than mutating
in place — immutability is a key feature of TEA.

**View**: A function `(Model) -> Widget` that renders the current state.

```rust
enum Message {
    Increment,
    Decrement,
    Quit,
}

fn update(model: Model, msg: Message) -> Model {
    match msg {
        Message::Increment => Model { counter: model.counter + 1, ..model },
        Message::Decrement => Model { counter: model.counter.saturating_sub(1), ..model },
        Message::Quit => Model { running: false, ..model },
    }
}
```

Returning a `Message` from `update()` allows the developer to reason about the app as a
**Finite State Machine**: a state + an event → a new state.

## Component Pattern

For complex applications, decompose the UI into components, each with:
- Its own `State` struct
- Render methods that accept a sub-area
- An `update` method handling relevant messages

The root `App` composes components and routes messages.

## Tick-Based Timing

For animations or periodic updates, use a tick mechanism:

```rust
let tick_rate = Duration::from_millis(50); // 20 fps
let mut last_tick = Instant::now();

loop {
    let timeout = tick_rate.saturating_sub(last_tick.elapsed());
    if event::poll(timeout)? {
        handle_events();
    }
    if last_tick.elapsed() >= tick_rate {
        on_tick();
        last_tick = Instant::now();
    }
    terminal.draw(|frame| render(frame))?;
}
```

## Architecture Decision Summary

| Concern | Recommended Approach |
|---|---|
| State container | `App` struct (or `Model` in TEA) |
| Rendering | `terminal.draw(\|frame\| render(frame))` |
| Widget rendering | `frame.render_widget(widget, area)` |
| Event handling | `crossterm::event::read()` or `poll()` |
| State updates | Methods on `App`, or pure `update(model, msg)` in TEA |
| Async | `tokio::sync::mpsc` + `EventStream` |
| Terminal setup | `ratatui::init()` / `ratatui::restore()` |
