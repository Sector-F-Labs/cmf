//! Shared rendering context and state management

use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub enum FormattingState {
    Bold,
    Italic,
    Link,
}

/// Shared context for rendering markdown elements with state tracking
pub struct RenderContext {
    pub output: String,
    pub formatting_stack: VecDeque<FormattingState>,
    pub pending_newlines: usize,
    #[allow(dead_code)]
    pub use_colors: bool,
}

impl RenderContext {
    pub fn new(use_colors: bool) -> Self {
        Self {
            output: String::new(),
            formatting_stack: VecDeque::new(),
            pending_newlines: 0,
            use_colors,
        }
    }

    pub fn push_str(&mut self, s: &str) {
        self.output.push_str(s);
        self.pending_newlines = 0;
    }

    pub fn push_newline(&mut self) {
        self.output.push('\n');
        self.pending_newlines += 1;
    }

    pub fn ensure_newline(&mut self) {
        if !self.output.is_empty() && !self.output.ends_with('\n') {
            self.push_newline();
        }
    }

    pub fn ensure_blank_line(&mut self) {
        if !self.output.is_empty() && !self.output.ends_with("\n\n") && self.pending_newlines < 2 {
            self.push_newline();
        }
    }

    pub fn into_output(self) -> String {
        self.output.trim_end().to_string() + "\n"
    }
}
