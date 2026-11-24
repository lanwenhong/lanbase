use serde::de::DeserializeOwned;
use std::{fs, io, path::Path};

/// Reads a UTF-8 TOML file into an application-defined configuration type.
pub fn load<T: DeserializeOwned>(path: impl AsRef<Path>) -> io::Result<T> {
    let path = path.as_ref();
    let source = fs::read_to_string(path)
        .map_err(|e| io::Error::new(e.kind(), format!("{}: {e}", path.display())))?;
    toml::from_str(&source).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("{}: {e}", path.display()),
        )
    })
}
