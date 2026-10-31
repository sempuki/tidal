// tidal-rules: CLS-25
// tidal-expect: reject
// tidal-error-clang: but function is not marked volatile
// tidal-error-gcc: passing .volatile S. as .this. argument discards qualifiers
// tidal-error-edg: type qualifiers that are not compatible with the member function
// tidal-note: Gap review 2026-10-31.
struct S { void f(); };
void g(volatile S& s) { s.f(); }
