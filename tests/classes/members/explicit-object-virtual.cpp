// tidal-rules: CLS-14
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
struct S { virtual void f(this S&); };
