//! Blockquote renderer with vertical bar prefix

use crate::terminal_renderer::context::RenderContext;
use crate::terminal_renderer::element_renderer::ElementRenderer;

/// Renders blockquotes with vertical bar prefix
pub struct BlockquoteRenderer {
    lines: Vec<String>,
    current_line: String,
}

impl BlockquoteRenderer {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            current_line: String::new(),
        }
    }

    fn add_prefix_to_lines(text: &str) -> String {
        text.lines()
            .map(|line| format!("▌ {}", line))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl ElementRenderer for BlockquoteRenderer {
    fn start(&mut self, _: &mut RenderContext) {
        self.lines.clear();
        self.current_line.clear();
    }

    fn handle_text(&mut self, text: &str, _: &mut RenderContext) {
        self.current_line.push_str(text);
    }

    fn handle_soft_break(&mut self, _: &mut RenderContext) {
        self.lines.push(self.current_line.clone());
        self.current_line.clear();
    }

    fn handle_hard_break(&mut self, _: &mut RenderContext) {
        self.lines.push(self.current_line.clone());
        self.current_line.clear();
    }

    fn end(&mut self, _: &mut RenderContext) -> Option<String> {
        if !self.current_line.is_empty() {
            self.lines.push(self.current_line.clone());
        }

        let full_text = self.lines.join("\n");
        Some(Self::add_prefix_to_lines(&full_text))
    }
}
