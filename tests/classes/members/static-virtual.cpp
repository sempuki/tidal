// tidal-rules: CLS-27
// tidal-expect: reject
// tidal-error-clang: virtual. can only appear on non-static member functions
// tidal-error-gcc: cannot be declared both .virtual. and .static
// tidal-error-edg: only nonstatic member functions may be virtual
// tidal-note: Gap review 2026-10-30.
struct S { static virtual void f(); };
