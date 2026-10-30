// tidal-rules: CLS-3
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
enum Color { Red, Green };
struct S { static const Color c = Green; };
static_assert(S::c == Green);
