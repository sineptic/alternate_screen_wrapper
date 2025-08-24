#[cfg(feature = "crossterm")]
pub mod crossterm;

#[cfg(feature = "unix")]
pub mod unix {
    use std::{
        io::{Write, stdout},
        panic::{set_hook, take_hook},
        sync::Mutex,
    };

    use rustix::termios::Termios;

    static ORIGINAL_MODE: Mutex<Option<Termios>> = Mutex::new(None);

    #[non_exhaustive]
    pub struct AlternateScreenOnStdout;
    impl AlternateScreenOnStdout {
        pub fn enter() -> std::io::Result<Option<Self>> {
            let mut mode = ORIGINAL_MODE.lock().unwrap_or_else(|err| err.into_inner());
            if mode.is_some() {
                return Ok(None);
            }

            init_panic_hook();
            let mut stdout = stdout();
            stdout.write_all(b"\x1B[?1049h")?;
            let mut termios = rustix::termios::tcgetattr(&stdout)?;
            *mode = Some(termios.clone());
            termios.make_raw();
            rustix::termios::tcsetattr(stdout, rustix::termios::OptionalActions::Now, &termios)?;
            Ok(Some(Self))
        }
    }
    impl Drop for AlternateScreenOnStdout {
        fn drop(&mut self) {
            if let Err(err) = restore_terminal() {
                eprintln!("Error: {err}");
            }
        }
    }

    pub fn restore_terminal() -> std::io::Result<()> {
        let mut stdout = stdout();
        let original_mode = ORIGINAL_MODE.lock().unwrap_or_else(|err| err.into_inner());
        if let Some(ref original_mode) = *original_mode {
            rustix::termios::tcsetattr(
                &stdout,
                rustix::termios::OptionalActions::Now,
                original_mode,
            )?;
        }
        stdout.write_all(b"\x1B[?1049l")?;
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
}
