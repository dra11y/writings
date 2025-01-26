mod gleanings_visitor;

#[cfg(feature = "_visitors")]
pub use gleanings_visitor::GleaningsVisitor;

mod gleanings_paragraph;
pub use gleanings_paragraph::GleaningsParagraph;
