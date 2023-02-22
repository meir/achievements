use std::io;
#[cfg(windows)] use winres::{WindowsResource, Build};

fn main() -> io::Result<()> {
    #[cfg(windows)] {
        Build::new().compile("tray-example.rc").unwrap();
        WindowsResource::new()
            // This path can be absolute, or relative to your crate root.
            .set_icon("assets/achievements.ico")
            .compile()?;
    }
    Ok(())
}