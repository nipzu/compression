use std::fmt;
use std::io;

pub enum ProgramError {
    FileReadError(io::Error, std::path::PathBuf),
    FileWriteError(io::Error, std::path::PathBuf),
    InvalidArgumentsError,
}

impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProgramError::FileReadError(e, path) => match e.kind() {
                io::ErrorKind::NotFound => write!(f, "File {} was not found", path.to_string_lossy()),
                io::ErrorKind::PermissionDenied => write!(f, "No permission to read file {}", path.to_string_lossy()),
                _ => write!(f, "An unknown error occurred while trying to read file {}", path.to_string_lossy()),
            },
            ProgramError::FileWriteError(e, path) => match e.kind() {
                io::ErrorKind::NotFound => write!(f, "File {} was not found", path.to_string_lossy()),
                io::ErrorKind::PermissionDenied => write!(f, "No permission to write to file {}", path.to_string_lossy()),
                _ => write!(f, "An unknown error occurred while trying to write to file {}", path.to_string_lossy()),
            },
            ProgramError::InvalidArgumentsError => write!(f, "Invalid arguments; please use [c(ompress) / d(ecompress)] [input_path] [output_path]"),
        }
    }
}
