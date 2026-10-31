// tidal-rules: JMP-12
// tidal-expect: reject
// tidal-error-clang: constructor .S. must not return void expression
// tidal-error-gcc: returning a value from a constructor
// tidal-error-edg: constructor or destructor may not return a value
// tidal-note: Gap review 2026-10-31 (third).
void nothing();
struct S { S() { return nothing(); } };
