//! Source files, spans, and the include stack.
//!
//! A token's span names the file that holds it, with its line and column
//! in that file. A file that `#include` brings in records the span of the
//! directive, so a diagnostic can still give the line of the translation
//! unit's main file, as the old front end printed it.

use std::path::{Path, PathBuf};

/// An index into the `SourceMap`. File 0 is the main file of the unit.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct FileId(pub u32);

/// A place in the source: a file, a line, and a column, each from 1.
/// A token that a `#define` name expands to takes the span of the name.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Span {
    pub file: FileId,
    pub line: u32,
    pub col: u32,
}

impl Span {
    pub fn new(file: FileId, line: u32, col: u32) -> Span {
        Span { file, line, col }
    }
}

/// One file of the unit, and where it was included.
#[derive(Debug, Clone)]
pub struct SourceFile {
    pub path: Option<PathBuf>,
    /// The span of the `#include` line that brought the file in, or
    /// `None` for the main file.
    pub included_at: Option<Span>,
}

/// The files of one translation unit.
#[derive(Debug, Default, Clone)]
pub struct SourceMap {
    pub files: Vec<SourceFile>,
}

impl SourceMap {
    /// A map whose main file has the given path.
    pub fn new(main: Option<&Path>) -> SourceMap {
        SourceMap { files: vec![SourceFile { path: main.map(Path::to_path_buf), included_at: None }] }
    }

    /// Record a file that the directive at `at` includes.
    pub fn add(&mut self, path: &Path, at: Span) -> FileId {
        self.files.push(SourceFile { path: Some(path.to_path_buf()), included_at: Some(at) });
        FileId(self.files.len() as u32 - 1)
    }

    pub fn path(&self, f: FileId) -> Option<&Path> {
        self.files.get(f.0 as usize).and_then(|s| s.path.as_deref())
    }

    /// The line in the main file: the span's own line for the main file,
    /// and the line of the outermost `#include` for a header.
    pub fn unit_line(&self, s: Span) -> u32 {
        let mut s = s;
        let mut guard = 0;
        while let Some(at) = self.files.get(s.file.0 as usize).and_then(|f| f.included_at) {
            s = at;
            guard += 1;
            if guard > 1000 {
                break;
            }
        }
        s.line
    }

    /// The include stack of a span, innermost first.
    pub fn stack(&self, s: Span) -> Vec<Span> {
        let mut out = vec![s];
        let mut cur = s;
        while let Some(at) = self.files.get(cur.file.0 as usize).and_then(|f| f.included_at) {
            out.push(at);
            cur = at;
            if out.len() > 1000 {
                break;
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_line_follows_includes() {
        let mut m = SourceMap::new(Some(Path::new("main.cpp")));
        let a = m.add(Path::new("a.h"), Span::new(FileId(0), 3, 1));
        let b = m.add(Path::new("b.h"), Span::new(a, 7, 1));
        assert_eq!(m.unit_line(Span::new(FileId(0), 9, 2)), 9);
        assert_eq!(m.unit_line(Span::new(b, 2, 5)), 3);
        assert_eq!(m.stack(Span::new(b, 2, 5)).len(), 3);
        assert_eq!(m.path(a), Some(Path::new("a.h")));
    }
}
