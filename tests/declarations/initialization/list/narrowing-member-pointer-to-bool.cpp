// tidal-rules: LIST-2
// tidal-expect: reject
// tidal-error-clang: cannot be narrowed
// tidal-error-gcc: narrowing conversion of
// tidal-error-edg: invalid narrowing conversion
// tidal-note: Gap review 2026-10-30.
struct S { int m; };
bool b{&S::m};
