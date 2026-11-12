//! Prototypes of Propose designs (`frontend/DESIGN-rewrite.md`, section 7).
//! A prototype lives in `proposals/<id>.rs` and one line of `REGISTRY`
//! registers it. It runs only under `--proposal <id>` and attaches only
//! at the extension points. No page module imports this module.
//!
//! P22 follows X24's design (decision 0065, item RW2).

pub mod p22;

use crate::sem::check::Hooks;
use crate::syntax::Alt;

/// A registered prototype: its ID, its alternatives at the nonterminal
/// point, and the hooks it attaches to analysis (the node-check point
/// and the `Ext` node point, build step 3). Rule overrides and
/// definition checks come with steps 4 and 5.
pub struct Prototype {
    pub id: &'static str,
    pub alternatives: &'static [Alt],
    pub hooks: fn() -> Hooks,
}

pub const REGISTRY: &[Prototype] = &[Prototype { id: "P22", alternatives: p22::ALTERNATIVES, hooks: p22::hooks }];

/// The prototype with this ID.
pub fn find(id: &str) -> Option<&'static Prototype> {
    REGISTRY.iter().find(|p| p.id.eq_ignore_ascii_case(id))
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_registry_finds_p22_and_refuses_unknown_ids() {
        assert_eq!(super::find("p22").map(|p| p.id), Some("P22"));
        assert!(super::find("P99").is_none());
    }
}
