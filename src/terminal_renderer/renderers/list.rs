//! List renderer for ordered and unordered lists

use crate::terminal_renderer::context::RenderContext;
use crate::terminal_renderer::element_renderer::ElementRenderer;

/// Renders lists with bullets or numbers
pub struct ListRenderer {
    #[allow(dead_code)]
    depth: usize,
    is_ordered: bool,
    item_indices: Vec<usize>,
    in_item: bool,
    buffer: String,
}

impl ListRenderer {
    pub fn new(ordered: bool, depth: usize) -> Self {
        Self {
            depth,
            is_ordered: ordered,
            item_indices: if ordered { vec![0] } else { Vec::new() },
            in_item: false,
            buffer: String::new(),
        }
    }

    pub fn start_item(&mut self, output: &mut String, depth: usize) {
        self.in_item = true;

        // Add indentation
        for _ in 0..(depth - 1) {
            output.push_str("  ");
        }

        // Add bullet or number
        if self.is_ordered {
            if let Some(idx) = self.item_indices.last_mut() {
                *idx += 1;
                output.push_str(&format!("{}. ", idx));
            }
        } else {
            output.push_str("• ");
        }
    }

    #[allow(dead_code)]
    fn end_item(&mut self) {
        self.in_item = false;
    }
}

impl ElementRenderer for ListRenderer {
    fn start(&mut self, _: &mut RenderContext) {
        self.buffer.clear();
    }

    fn handle_text(&mut self, text: &str, _: &mut RenderContext) {
        self.buffer.push_str(text);
    }

    fn handle_soft_break(&mut self, _: &mut RenderContext) {
        self.buffer.push(' ');
    }

    fn handle_hard_break(&mut self, _: &mut RenderContext) {
        self.buffer.push('\n');
    }

    fn end(&mut self, _: &mut RenderContext) -> Option<String> {
        // ListRenderer is handled differently in main render loop
        None
    }
}
