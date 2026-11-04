// tidal-rules: CONV-10
// tidal-expect: reject
// tidal-error-clang: different exception specifications
// tidal-error-gcc: cannot convert .void \(S::\*\)\(\). to .void \(S::\*\)\(\) noexcept.
// tidal-error-edg: cannot be used to initialize an entity of type .void \(S::\*\)\(\) noexcept
// tidal-note: Gap review 2026-10-31 (third).
struct S { void f(); };
void (S::* p)() noexcept = &S::f;
