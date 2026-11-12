//! The prototype of P22, labeled `break` and `continue`, as X24 designs
//! it (`reports/clean-room/X24/report.md`, sections 1 and 2;
//! `frontend/DESIGN-rewrite.md` section 7). It runs only under
//! `--proposal P22`, and it attaches at two points:
//!
//! - the nonterminal point: the alternatives `"break" identifier ";"` and
//!   `"continue" identifier ";"` at `jump_stmt` (X24's amended
//!   `jump_stmt`, JMP-17 withdrawn);
//! - the `Ext` node point: the check of that node, which names X24's new
//!   rules X24:new-1 and X24:new-2 and gives an ordinary `Break` or
//!   `Continue` with the labeled statement as its target.
//!
//! X24:new-3, the run-time rule, needs no code: the machine already
//! passes a `Break` or `Continue` to the statement it names, and ends the
//! objects of each block left on the way (JMP-9, LOOP-9). The "run: none"
//! of section 7 holds for the variables of a statement's own scope only
//! once X24-1 is answered; the machine ends them with their statement's
//! block today.

use crate::expressions::primary::syntax::identifier;
use crate::sem::check::{Check, Hooks};
use crate::sem::tree::{Stmt, SK};
use crate::statements::jumps::check::enclosing;
use crate::syntax::{Alt, Node, PResult, Parser};

/// The rules of X24's change table that the prototype names, with their
/// kinds from the "New class" column (X24 section 2).
pub const RULES: &[(&str, &str)] = &[("X24:new-1", "reject"), ("X24:new-2", "reject"), ("X24:new-3", "required")];

/// `jump_stmt: "break" identifier ";" | "continue" identifier ";"`: the
/// alternatives of X24 section 1.2.
pub fn labeled_jump(p: &mut Parser) -> PResult<Node> {
    let sp = p.span();
    if !(p.at("break") || p.at("continue")) {
        return Err(p.fail("'break' or 'continue'"));
    }
    let kw = p.bump();
    let label = identifier(p)?;
    let semi = p.expect(";")?;
    Ok(p.node("jump_stmt", sp, vec![kw, label, semi]))
}

pub const ALTERNATIVES: &[Alt] = &[Alt { nonterminal: "jump_stmt", proposal: "P22", parse: labeled_jump }];

pub fn hooks() -> Hooks {
    Hooks { proposal: Some("P22"), node_checks: Vec::new(), ext_stmt: Some(check) }
}

/// The check of a labeled jump (X24 sections 1.3 to 1.5). The label must
/// mark a loop or `switch` that encloses the jump in the jump's own
/// function (X24:new-1, with X24's JMP-14 amendment), and the label of a
/// `continue` must mark a loop (X24:new-2).
pub fn check(cx: &mut Check, n: &Node) -> Stmt {
    let words = cx.words(n);
    let (kw, label) = (words.first().copied().unwrap_or(""), words.get(1).copied().unwrap_or(""));
    let span = n.span;
    let target = enclosing(cx).into_iter().rev().find(|t| t.labels.iter().any(|l| &**l == label));
    match target {
        None => {
            cx.error(span, "X24:new-1", format!("the label '{}' marks no loop or switch that encloses the jump in its function", label));
            Stmt::new(SK::Null, span)
        }
        Some(t) if kw == "continue" && !t.is_loop => {
            cx.error(span, "X24:new-2", format!("the label '{}' of a continue marks a switch", label));
            Stmt::new(SK::Null, span)
        }
        Some(t) => Stmt::new(if kw == "break" { SK::Break(t.id) } else { SK::Continue(t.id) }, span),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::foundations::translation::run::{run_program, RunOutcome};
    use crate::Outcome;
    use std::path::Path;

    fn check_with(src: &str) -> (Outcome, Option<crate::sem::Unit>) {
        let o = crate::Options { proposal: Some("P22"), ..Default::default() };
        crate::check_source(src, Path::new("t.cpp"), &o)
    }

    fn first_rule(src: &str) -> Option<&'static str> {
        match check_with(src).0 {
            Outcome::Reject(ds) => ds[0].rule,
            _ => None,
        }
    }

    #[test]
    fn the_forms_are_rejected_without_the_flag() {
        let src = "int f() { outer: for (int i = 0; i < 3; ++i) for (;;) break outer; return 0; }";
        let (o, _) = crate::check_source(src, Path::new("t.cpp"), &crate::Options::default());
        assert!(matches!(o, Outcome::Reject(ds) if ds[0].rule == Some("TR-12")));
    }

    #[test]
    fn find_of_section_1_1_is_a_constant() {
        let src = "constexpr int find(const int (&grid)[3][4], int target) {\n  int at = -1;\nrows:\n  for (int r = 0; r < 3; ++r)\n    for (int c = 0; c < 4; ++c)\n      if (grid[r][c] == target) {\n        at = r * 4 + c;\n        break rows;\n      }\n  return at;\n}\nconstexpr int g[3][4] = {{1, 2, 3, 4}, {5, 6, 7, 8}, {9, 7, 11, 12}};\nstatic_assert(find(g, 7) == 6);\n";
        assert!(matches!(check_with(src).0, Outcome::Accept));
    }

    #[test]
    fn continue_with_a_label_goes_on_with_the_outer_loop() {
        let src = "int main() {\n  int sum = 0;\nrows:\n  for (int r = 0; r < 3; ++r) {\n    for (int c = 0; c < 4; ++c) {\n      if (c == 2) continue rows;\n      sum += 1;\n    }\n  }\n  return sum;\n}\n";
        let (o, u) = check_with(src);
        assert!(matches!(o, Outcome::Accept));
        assert_eq!(run_program(&u.unwrap()), RunOutcome::Result(6));
    }

    #[test]
    fn the_new_rules_reject_their_forms() {
        // X24:new-2: a continue whose label marks a switch.
        assert_eq!(first_rule("int f(int m) { d: switch (m) { case 1: for (;;) continue d; } return 0; }"), Some("X24:new-2"));
        // X24:new-1: a label on a sibling loop, on a block, and none.
        assert_eq!(first_rule("int f() { a: for (;;) {} for (;;) break a; return 0; }"), Some("X24:new-1"));
        assert_eq!(first_rule("int f() { b: { for (;;) break b; } return 0; }"), Some("X24:new-1"));
        assert_eq!(first_rule("int f() { for (;;) break nowhere; return 0; }"), Some("X24:new-1"));
        // A chain of labels marks the loop at its end.
        assert!(matches!(check_with("int f() { first: second: for (;;) break first; return 0; }").0, Outcome::Accept));
    }

    #[test]
    fn the_rules_and_kinds_follow_x24s_change_table() {
        let report = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../reports/clean-room/X24/report.md")).unwrap();
        for (id, kind) in RULES {
            let row = report.lines().find(|l| l.starts_with(&format!("| {} |", id))).unwrap_or_else(|| panic!("no row for {}", id));
            let cells: Vec<&str> = row.split('|').map(str::trim).collect();
            assert_eq!(cells[4], *kind, "the new class of {}", id);
        }
    }
}
