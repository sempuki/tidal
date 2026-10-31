// tidal-rules: CLS-16
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
struct S { static thread_local int v; };
thread_local int S::v = 1;
