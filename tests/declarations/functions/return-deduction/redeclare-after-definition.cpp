// tidal-rules: RET-4
// tidal-expect: reject
// tidal-error-clang: differ only in their return type
// tidal-error-gcc: ambiguating new declaration
// tidal-error-edg: declaration is incompatible with
// tidal-note: Gap review 2026-10-30.
auto f() { return 1; }
int f();
