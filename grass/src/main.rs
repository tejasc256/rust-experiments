use clap::Parser;
use anyhow::{Context, Result};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}


fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line).unwrap();
        }
    }
}


fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).
        with_context(|| format!("could not read file `{}`", args.path.display()))?;
    find_matches(&content, &args.pattern, &mut std::io::stdout());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FailingWriter;
    impl std::io::Write for FailingWriter {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "write failed"))
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn find_a_match() {
        let mut result = Vec::new();
        find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
        assert_eq!(result, b"lorem ipsum\n");
    }

    #[test]
    #[should_panic]
    fn find_a_match_error() {
        let writer = FailingWriter;
        find_matches("lorem ipsum\n", "lorem", writer);
    }
}
