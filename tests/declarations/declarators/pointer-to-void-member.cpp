// tidal-rules: DCL-12
// tidal-expect: reject
// tidal-error-clang: declared as a member pointer to void
// tidal-error-gcc: cannot declare pointer to .void. member
// tidal-error-edg: pointer to member of type void is not allowed
// tidal-note: Gap review 2026-10-30.
struct S {};
void S::* pm;
