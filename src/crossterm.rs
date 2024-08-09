use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::stdout;

#[non_exhaustive]
pub struct AlternateScreen {}
impl AlternateScreen {
    pub fn enter() -> std::io::Result<Self> {
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        Ok(Self {})
    }
}
impl Drop for AlternateScreen {
    fn drop(&mut self) {
        let err1 = stdout().execute(LeaveAlternateScreen).map(|_| ());
        let err2 = disable_raw_mode();

        if let Err(err) = err1 {
            eprintln!("Error: {err}.");
        }
        if let Err(err) = err2 {
            eprintln!("Error: {err}.");
        }
    }
}
