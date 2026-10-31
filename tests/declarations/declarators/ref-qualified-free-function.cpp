// tidal-rules: DCL-7
// tidal-expect: reject
// tidal-error-clang: non-member function cannot have .&. qualifier
// tidal-error-gcc: non-member function .* cannot have ref-qualifier
// tidal-error-edg: ref-qualifier is not allowed here
// tidal-note: Gap review 2026-10-30.
void f() &;
