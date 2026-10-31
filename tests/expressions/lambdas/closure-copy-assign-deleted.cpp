// tidal-rules: LAM-11
// tidal-expect: reject
// tidal-error-clang: copy assignment operator is implicitly deleted
// tidal-error-gcc: use of deleted function .*operator=
// tidal-error-edg: operator=.* it is a deleted function
// tidal-note: Gap review 2026-10-31.
void f() { int x = 1; auto a = [x] { return x; }; auto b = a; b = a; }
