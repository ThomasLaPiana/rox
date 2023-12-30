use crate::models::{Docs, DocsKind};
use std::io::{stdout, Write};
use termimad::crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode, KeyEvent},
    queue,
    style::Color::*,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use termimad::MadSkin;
use termimad::*;

static KEYBINDINGS: &str = r#"
# Navigation Keybindings

| Key(s) | Action |
| :---: | :------: |
| q/Esc | Exit |
| k/Up | Scroll up one line|
| j/Down | Scroll down one line |
| g | Jump to top |
| G | Jump to Bottom |
| u | Page Up |
| d | Page Down |

------
"#;

pub fn display_docs(docs: &Docs) {
    match docs.kind {
        DocsKind::Markdown => {
            let markdown = std::fs::read_to_string(&docs.path).unwrap();
            run_app(&markdown).unwrap();
        }
        DocsKind::Text => {
            let contents = std::fs::read_to_string(&docs.path).unwrap();
            println!("{}", contents);
        }
        DocsKind::URL => {
            println!("> Opening '{}' in your browser...", docs.path);
            webbrowser::open(&docs.path).unwrap()
        }
    }
}

/// Build and Run the terminal application
/// Taken from -> https://github.com/Canop/termimad/blob/main/examples/scrollable/main.rs
fn run_app(docs: &str) -> Result<(), Error> {
    let mut w = stdout(); // we could also have used stderr
    let skin = make_skin();
    queue!(w, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    queue!(w, Hide)?; // hiding the cursor

    let markdown = format!("{}\n{}", KEYBINDINGS, docs);
    let mut view = MadView::from(markdown, view_area(), skin);
    loop {
        view.write_on(&mut w)?;
        w.flush()?;
        match event::read() {
            Ok(Event::Key(KeyEvent { code, .. })) => match code {
                // Negative number is up, Positive number is down
                KeyCode::Up => view.try_scroll_lines(-1),
                KeyCode::Down => view.try_scroll_lines(1),
                KeyCode::Char('k') => view.try_scroll_lines(-1),
                KeyCode::Char('j') => view.try_scroll_lines(1),

                KeyCode::Char('u') => view.try_scroll_lines(-30),
                KeyCode::Char('d') => view.try_scroll_lines(30),

                KeyCode::Char('g') => view.try_scroll_lines(-100000),
                KeyCode::Char('G') => view.try_scroll_lines(100000),

                KeyCode::Char('q') => break,
                KeyCode::Esc => break,
                _ => continue,
            },
            Ok(Event::Resize(..)) => {
                queue!(w, Clear(ClearType::All))?;
                view.resize(&view_area());
            }
            _ => {}
        }
    }
    terminal::disable_raw_mode()?;
    queue!(w, Show)?; // we must restore the cursor
    queue!(w, LeaveAlternateScreen)?;
    w.flush()?;
    Ok(())
}

fn make_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.table.align = Alignment::Center;
    skin.set_headers_fg(AnsiValue(178));
    skin.bold.set_fg(Yellow);
    skin.italic.set_fg(Magenta);
    skin.scrollbar.thumb.set_fg(AnsiValue(178));
    skin.code_block.align = Alignment::Center;
    skin
}

fn view_area() -> Area {
    let mut area = Area::full_screen();
    area.pad_for_max_width(120); // we don't want a too wide text column
    area
}
