// tidal-rules: MPTR-9
// tidal-expect: reject
// tidal-error-clang: with an rvalue of type .int \(\*\)\(const S &\).
// tidal-error-gcc: cannot convert .int \(\*\)\(const S&\). to .int \(S::\*\)\(\) const.
// tidal-error-edg: int \(\*\)\(const S &\). cannot be used to initialize
// tidal-note: Gap review 2026-10-31.
struct S { int get(this const S& self) { return 1; } };
int (S::* p)() const = &S::get;
