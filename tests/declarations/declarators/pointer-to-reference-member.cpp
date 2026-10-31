// tidal-rules: DCL-12
// tidal-expect: reject
// tidal-error-clang: declared as a member pointer to a reference
// tidal-error-gcc: cannot declare pointer to .int&
// tidal-error-edg: pointer to member of reference type is not allowed
// tidal-note: Gap review 2026-10-30.
struct S { int& r; };
int& S::* pm;
