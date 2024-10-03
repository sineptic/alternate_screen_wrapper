use crossterm::{
    event::{DisableBracketedPaste, EnableBracketedPaste},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::stdout,
    panic::{set_hook, take_hook},
};

#[non_exhaustive]
pub struct AlternateScreen {}
impl AlternateScreen {
    pub fn enter() -> std::io::Result<Self> {
        init_panic_hook();
        execute!(stdout(), EnterAlternateScreen)?;
        enable_raw_mode()?;
        Ok(Self {})
    }
    pub fn bracketed_paste(self) -> std::io::Result<Self> {
        execute!(stdout(), EnableBracketedPaste)?;
        Ok(self)
    }
}
impl Drop for AlternateScreen {
    fn drop(&mut self) {
        if let Err(err) = restore_tui() {
            eprintln!("Error: {err}");
        }
    }
}

fn restore_tui() -> std::io::Result<()> {
    disable_raw_mode()?;
    execute!(
        std::io::stdout(),
        LeaveAlternateScreen,
        DisableBracketedPaste
    )?;
    Ok(())
}

fn init_panic_hook() {
    let original_hook = take_hook();
    set_hook(Box::new(move |panic_info| {
        // intentionally ignore errors here since we're already in a panic
        let _ = restore_tui();
        original_hook(panic_info);
    }));
}
