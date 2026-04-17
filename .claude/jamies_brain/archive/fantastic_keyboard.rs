use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    style::{Color, Print, Stylize},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::io::{self, Write};
use std::time::{Duration, Instant};

// --- CONFIG ---
const COLOR_GLOW: Color = Color::Rgb { r: 0, g: 255, b: 255 }; // Cyan
const COLOR_IDLE: Color = Color::Rgb { r: 60, g: 60, b: 65 };  // Gunmetal

struct KeyPos { x: u16, y: u16 }
struct Glow { intensity: f32 }

// --- THE GLYPH REGISTRY (The "IP vs DNS" for your Alien Deck) ---
static GLYPH_MAP: Lazy<HashMap<char, char>> = Lazy::new(|| {
    let mut m = HashMap::new();
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890 ";
    for (i, c) in chars.chars().enumerate() {
        if c == ' ' { m.insert(c, ' '); continue; }
        // Use a high-variety offset across the 1000+ char Egyptian block (0x13000 - 0x1342E)
        let salt = (i as u32 * 137) % 1000; 
        let glyph = std::char::from_u32(0x13000 + salt).unwrap_or('?');
        m.insert(c, glyph);
    }
    m
});

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, cursor::Hide, Clear(ClearType::All))?;

    let key_map = create_tight_map();
    let mut active_glows: HashMap<char, Glow> = HashMap::new();
    let mut decrypted_display: Vec<(char, char, i32)> = Vec::new(); 
    let mut last_tick = Instant::now();

    // --- INITIALIZATION SCAN (Movie Effect) ---
    draw_static_shell(&mut stdout)?;
    for (c, _) in &key_map {
        active_glows.insert(*c, Glow { intensity: 1.0 });
        render_dynamic_elements(&mut stdout, &key_map, &active_glows, &decrypted_display)?;
        stdout.flush()?;
        std::thread::sleep(Duration::from_millis(15));
    }

    loop {
        let dt = last_tick.elapsed().as_secs_f32();
        last_tick = Instant::now();

        // 1. INPUT
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => break,
                        KeyCode::Char(' ') => {
                            active_glows.insert(' ', Glow { intensity: 1.5 });
                            decrypted_display.push((' ', ' ', 0));
                        }
                        KeyCode::Char(c) => {
                            let up_c = c.to_ascii_uppercase();
                            if key_map.contains_key(&up_c) {
                                active_glows.insert(up_c, Glow { intensity: 1.5 });
                                // Start scramble: (Target Glyph, Current Flicker, Timer)
                                let target = *GLYPH_MAP.get(&up_c).unwrap_or(&up_c);
                                decrypted_display.push((target, '?', 10));
                            }
                        }
                        KeyCode::Backspace => { decrypted_display.pop(); }
                        _ => {}
                    }
                }
            }
        }

        // 2. LOGIC: Decay + Scramble
        active_glows.retain(|_, glow| {
            glow.intensity -= 1.5 * dt;
            glow.intensity > -0.1 
        });

        for item in decrypted_display.iter_mut() {
            if item.2 > 0 {
                // Flickers through the ENTIRE block for maximum chaos
                item.1 = get_random_scramble_glyph();
                item.2 -= 1;
            } else { 
                item.1 = item.0; // Lock to the assigned session glyph
            }
        }

        // 3. RENDER (No flicker, delta-only)
        render_dynamic_elements(&mut stdout, &key_map, &active_glows, &decrypted_display)?;
        stdout.flush()?;
    }

    execute!(stdout, LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

fn create_tight_map() -> HashMap<char, KeyPos> {
    let mut m = HashMap::new();
    // Calibrated for the ASCII Shell rows
    let rows = [
        ("QWERTYUIOP", 10, 7),
        ("ASDFGHJKL", 11, 9),
        ("ZXCVBNM", 11, 11),
    ];
    for (chars, start_x, y) in rows {
        for (i, c) in chars.chars().enumerate() {
            m.insert(c, KeyPos { x: start_x + (i as u16 * 4), y });
        }
    }
    m
}

fn draw_static_shell(stdout: &mut io::Stdout) -> io::Result<()> {
    let y_base = 6;
    let shell = vec![
        "  ---------------------------------------------------------------------------",
        "  | Tab |   |   |   |   |   |   |   |   |   |   | [{| ]}| \\|    | Del  |End  |",
        "  ---------------------------------------------------------------------------",
        "  | Caps |   |   |   |   |   |   |   |   |   | ;:| '\"| Enter    |      | 4 <|",
        "  ---------------------------------------------------------------------------",
        "  | Shft |   |   |   |   |   |   |   |   |   | /?| Shft |   ^  |      | 1 E|",
        "  ---------------------------------------------------------------------------",
    ];
    for (i, line) in shell.iter().enumerate() {
        execute!(stdout, cursor::MoveTo(0, y_base + i as u16), Print(line.dark_grey()))?;
    }
    Ok(())
}

fn render_dynamic_elements(stdout: &mut io::Stdout, m: &HashMap<char, KeyPos>, glows: &HashMap<char, Glow>, msg: &[(char, char, i32)]) -> io::Result<()> {
    // 1. Decrypted Stream Area
    execute!(stdout, cursor::MoveTo(5, 2), Clear(ClearType::CurrentLine))?;
    print!("{} ", "ENIGMA_STREAM >".dark_grey());
    
    // De-Jammed: 2 spaces between wide glyphs
    let visible = msg.iter().rev().take(15).collect::<Vec<_>>();
    for item in visible.into_iter().rev() {
        let color = if item.2 > 0 { Color::White } else { COLOR_GLOW };
        print!("{}  ", item.1.with(color).bold());
    }

    // 2. Keyboard Glows
    for (c, pos) in m {
        let intensity = glows.get(c).map(|g| g.intensity).unwrap_or(0.0);
        let color = if intensity > 1.0 { Color::White } 
                    else if intensity > 0.0 { COLOR_GLOW } 
                    else { COLOR_IDLE };
        execute!(stdout, cursor::MoveTo(pos.x, pos.y), Print(c.with(color).bold()))?;
    }
    Ok(())
}

fn get_random_scramble_glyph() -> char {
    let nanos = Instant::now().elapsed().as_nanos() as u32;
    let offset = nanos % 1070;
    std::char::from_u32(0x13000 + offset).unwrap_or('?')
}

