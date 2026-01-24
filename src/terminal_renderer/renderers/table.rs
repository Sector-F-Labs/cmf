//! Table renderer with box-drawing characters

use crate::terminal_renderer::context::RenderContext;
use crate::terminal_renderer::element_renderer::ElementRenderer;

/// Renders markdown tables with box-drawing characters
pub struct TableRenderer {
    rows: Vec<Vec<String>>,
    current_row: Vec<String>,
    current_cell: String,
}

impl TableRenderer {
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            current_row: Vec::new(),
            current_cell: String::new(),
        }
    }

    fn add_cell(&mut self, cell: String) {
        self.current_row.push(cell);
    }

    fn finish_row(&mut self) {
        if !self.current_row.is_empty() {
            self.rows.push(self.current_row.clone());
            self.current_row.clear();
        }
    }

    pub fn render_table(&self) -> String {
        if self.rows.is_empty() {
            return String::new();
        }

        let num_cols = self.rows.iter().map(|r| r.len()).max().unwrap_or(0);
        let mut col_widths = vec![0; num_cols];

        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                col_widths[i] = col_widths[i].max(cell.len());
            }
        }

        let mut output = String::new();

        // Top border
        output.push('┌');
        for (i, width) in col_widths.iter().enumerate() {
            output.push_str(&"─".repeat(width + 2));
            if i < col_widths.len() - 1 {
                output.push('┬');
            }
        }
        output.push_str("┐\n");

        // Rows
        for (row_idx, row) in self.rows.iter().enumerate() {
            output.push('│');
            for (col_idx, cell) in row.iter().enumerate() {
                output.push(' ');
                output.push_str(&format!("{:<width$}", cell, width = col_widths[col_idx]));
                output.push(' ');
                output.push('│');
            }
            output.push('\n');

            // Add separator line between rows (or after header)
            if row_idx < self.rows.len() - 1 {
                output.push('├');
                for (i, width) in col_widths.iter().enumerate() {
                    output.push_str(&"─".repeat(width + 2));
                    if i < col_widths.len() - 1 {
                        output.push('┼');
                    }
                }
                output.push_str("┤\n");
            }
        }

        // Bottom border
        output.push('└');
        for (i, width) in col_widths.iter().enumerate() {
            output.push_str(&"─".repeat(width + 2));
            if i < col_widths.len() - 1 {
                output.push('┴');
            }
        }
        output.push_str("┘");

        output
    }

    pub fn start_cell(&mut self) {
        self.current_cell.clear();
    }

    pub fn end_cell(&mut self) {
        self.add_cell(self.current_cell.clone());
        self.current_cell.clear();
    }

    pub fn start_row(&mut self) {
        self.current_row.clear();
    }

    pub fn end_row(&mut self) {
        self.finish_row();
    }
}

impl ElementRenderer for TableRenderer {
    fn start(&mut self, _: &mut RenderContext) {
        self.rows.clear();
        self.current_row.clear();
        self.current_cell.clear();
    }

    fn handle_text(&mut self, text: &str, _: &mut RenderContext) {
        self.current_cell.push_str(text);
    }

    fn handle_soft_break(&mut self, _: &mut RenderContext) {
        self.current_cell.push(' ');
    }

    fn handle_hard_break(&mut self, _: &mut RenderContext) {
        self.current_cell.push('\n');
    }

    fn end(&mut self, _: &mut RenderContext) -> Option<String> {
        Some(self.render_table())
    }
}
