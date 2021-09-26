use derive_new::new;
use std::ffi::OsStr;

///
/// Construct a cargo instance from a list of arguments
/// ```
/// cargo!("run", "--release").run().unwrap();
/// ```
#[macro_export]
macro_rules! cargo {
    ( $($e : expr),* ) => {
        Cargo::new( [$($e),*] )
    };
}

#[derive(new, Debug)]
pub struct Cargo<S: AsRef<OsStr>, T: IntoIterator<Item = S>> {
    args: T,
}

impl<S: AsRef<OsStr>, T: IntoIterator<Item = S>> Cargo<S, T> {
    pub fn run(self) -> std::io::Result<()> {
        std::process::Command::new("cargo")
            .args(self.args)
            .spawn()?
            .wait()?;
        Ok(())
    }
    pub fn to_process(self) -> std::process::Command {
        let mut cmd = std::process::Command::new("cargo");
        cmd.args(self.args);
        cmd
    }
}

impl Cargo<String, Vec<String>> {
    pub fn from_args() -> Cargo<String, Vec<String>> {
        let mut args = std::env::args().collect::<Vec<_>>();
        // remove path of execution
        args.remove(0);
        Cargo::new(args)
    }
}

impl<S: AsRef<OsStr>, T: IntoIterator<Item = S>> From<Cargo<S, T>> for std::process::Command {
    fn from(c: Cargo<S, T>) -> Self {
        c.to_process()
    }
}
