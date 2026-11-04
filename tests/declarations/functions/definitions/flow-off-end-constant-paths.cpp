// tidal-rules: FN-7
// tidal-expect: accept
// tidal-flags: -Werror -Wreturn-type
// tidal-note: A loop whose condition is a constant true and that has no break does not complete. An if with a constant condition takes only the branch that the condition selects. No function here can reach its end, so no warning is given.
int forever_while() { while (true) {} }
int forever_for() { for (;;) {} }
int forever_do() { do {} while (true); }
int forever_one() { while (1) {} }
int selected_if() { if (true) return 1; }
int selected_constexpr() { if constexpr (true) return 1; }
int selected_else() { if constexpr (false) {} else return 1; }
int inner_break(int n) { for (;;) { switch (n) { case 0: break; } } }
