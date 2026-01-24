//! Trait for composable element renderers

use crate::terminal_renderer::context::RenderContext;

/// Trait for rendering specific markdown element types
pub trait ElementRenderer {
    fn start(&mut self, context: &mut RenderContext);
    fn handle_text(&mut self, text: &str, context: &mut RenderContext);
    fn handle_soft_break(&mut self, context: &mut RenderContext);
    fn handle_hard_break(&mut self, context: &mut RenderContext);
    fn end(&mut self, context: &mut RenderContext) -> Option<String>;
}
