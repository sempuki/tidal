// tidal-rules: DCL-14
// tidal-expect: reject
// tidal-error-clang: is missing exception specification
// tidal-error-gcc: has a different exception specifier
// tidal-error-edg: incompatible with previous function
// tidal-note: Gap review 2026-10-30.
void f() noexcept;
void f() {}
