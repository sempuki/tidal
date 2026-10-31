// tidal-rules: UN-11
// tidal-expect: reject
// tidal-error-clang: arithmetic on a pointer to the function type
// tidal-error-gcc: incrementing a pointer of type
// tidal-error-edg: pointer to a complete object type
// tidal-note: Gap review 2026-10-31.
void g();
void f() { void (*p)() = g; ++p; }
