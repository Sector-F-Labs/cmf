//! Code block renderer

use crate::terminal_renderer::context::RenderContext;
use crate::terminal_renderer::element_renderer::ElementRenderer;

/// Renders code blocks with 4-space indentation
pub struct CodeBlockRenderer {
    buffer: String,
}

impl CodeBlockRenderer {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    fn render_code_block(&self, code: &str) -> String {
        let lines: Vec<&str> = code.lines().collect();
        if lines.is_empty() {
            return String::new();
        }

        let mut output = String::new();

        // Indent each line by 4 spaces
        for line in lines {
            output.push_str("    ");
            output.push_str(line);
            output.push('\n');
        }

        // Remove trailing newline (will be added by caller)
        if output.ends_with('\n') {
            output.pop();
        }

        output
    }
}

impl ElementRenderer for CodeBlockRenderer {
    fn start(&mut self, _: &mut RenderContext) {
        self.buffer.clear();
    }

    fn handle_text(&mut self, text: &str, _: &mut RenderContext) {
        self.buffer.push_str(text);
    }

    fn handle_soft_break(&mut self, _: &mut RenderContext) {
        self.buffer.push('\n');
    }

    fn handle_hard_break(&mut self, _: &mut RenderContext) {
        self.buffer.push('\n');
    }

    fn end(&mut self, _context: &mut RenderContext) -> Option<String> {
        Some(self.render_code_block(&self.buffer))
    }
}
