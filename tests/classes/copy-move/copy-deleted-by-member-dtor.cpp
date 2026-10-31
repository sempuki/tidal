// tidal-rules: CPY-7
// tidal-expect: reject
// tidal-error-clang: implicitly-deleted|call to deleted
// tidal-error-gcc: use of deleted function
// tidal-error-edg: deleted function
// tidal-note: Gap review 2026-10-31.
struct M { ~M() = delete; };
struct S { M m; S(); S(const S&) = default; };
void f(S& a) { S b(a); (void)b; }
