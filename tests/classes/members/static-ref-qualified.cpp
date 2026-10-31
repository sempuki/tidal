// tidal-rules: CLS-27
// tidal-expect: reject
// tidal-error-clang: static member function cannot have .&. qualifier
// tidal-error-gcc: static member function .* cannot have ref-qualifier
// tidal-error-edg: ref-qualifier is not allowed here
// tidal-note: Gap review 2026-10-30.
struct S { static void f() &; };
