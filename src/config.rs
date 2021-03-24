use fluid_let::fluid_let;
use rustc_span::{source_map::{SourceFile, SourceMap}, BytePos, Span};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct Range {
  pub start_line: usize,
  pub start_col: usize,
  pub end_line: usize,
  pub end_col: usize,
}

impl Range {
  pub fn line(line: usize, start: usize, end: usize) -> Range {
    Range {
      start_line: line,
      start_col: start,
      end_line: line,
      end_col: end,
    }
  }

  pub fn substr(&self, s: &str) -> String {
    let lines = s.split("\n").collect::<Vec<_>>();
    if self.start_line != self.end_line {
      unimplemented!()
    } else {
      lines[self.start_line][self.start_col..self.end_col].to_owned()
    }
  }
}

impl Range {
  pub fn from_span(span: Span, source_map: &SourceMap) -> Self {
    let lines = source_map.span_to_lines(span).unwrap();
    let start_line = lines.lines.first().unwrap();
    let end_line = lines.lines.last().unwrap();
    Range {
      start_line: start_line.line_index,
      start_col: start_line.start_col.0,
      end_line: end_line.line_index,
      end_col: end_line.end_col.0,
    }
  }

  pub fn to_span(&self, source_file: &SourceFile) -> Span {
    let start_pos = source_file.line_bounds(self.start_line).start + BytePos(self.start_col as u32);
    let end_pos = source_file.line_bounds(self.end_line).start + BytePos(self.end_col as u32);
    Span::with_root_ctxt(start_pos, end_pos)
  }
}

#[derive(Debug)]
pub struct Config {
  pub path: String,
  pub range: Range,
  pub debug: bool,
}

fluid_let!(pub static CONFIG: Config);
