use std::{
    io::stdout,
    panic::{set_hook, take_hook},
};

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
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
    #[cfg(feature = "crossterm-bracketed-paste")]
    pub fn bracketed_paste(self) -> std::io::Result<Self> {
        execute!(stdout(), crossterm::event::EnableBracketedPaste)?;
        Ok(self)
    }
}
impl Drop for AlternateScreen {
    fn drop(&mut self) {
        if let Err(err) = restore_terminal() {
            eprintln!("Error: {err}");
        }
    }
}

pub fn restore_terminal() -> std::io::Result<()> {
    disable_raw_mode()?;

    #[cfg(feature = "crossterm-bracketed-paste")]
    execute!(
        std::io::stdout(),
        LeaveAlternateScreen,
        crossterm::event::DisableBracketedPaste
    )?;
    #[cfg(not(feature = "crossterm-bracketed-paste"))]
    execute!(std::io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

fn init_panic_hook() {
    let original_hook = take_hook();
    set_hook(Box::new(move |panic_info| {
        // intentionally ignore errors here since we're already in a panic
        let _ = restore_terminal();
        original_hook(panic_info);
    }));
}
