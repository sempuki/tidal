//! Prototypes of Propose designs (`frontend/DESIGN-rewrite.md`, section 7).
//! A prototype lives in `proposals/<id>.rs` and one line of `REGISTRY`
//! registers it. It runs only under `--proposal <id>` and attaches only
//! at the extension points. No page module imports this module.
//!
//! The registry is empty: item E of decision 0064 gives P22 its design
//! (X24) before a prototype follows it.

use crate::syntax::Alt;

/// A registered prototype: its ID and its alternatives at the
/// nonterminal point. The other points (`Ext` nodes, rule overrides,
/// definition checks, node checks) come with the steps that build them.
pub struct Prototype {
    pub id: &'static str,
    pub alternatives: &'static [Alt],
}

pub const REGISTRY: &[Prototype] = &[];

/// The prototype with this ID.
pub fn find(id: &str) -> Option<&'static Prototype> {
    REGISTRY.iter().find(|p| p.id.eq_ignore_ascii_case(id))
}

#[cfg(test)]
mod tests {
    #[test]
    fn registry_is_empty_and_unknown_ids_are_refused() {
        assert!(super::REGISTRY.is_empty());
        assert!(super::find("P22").is_none());
    }
}
