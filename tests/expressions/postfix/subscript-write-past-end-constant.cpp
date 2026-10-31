// tidal-rules: PFX-4
// tidal-expect: reject
// tidal-error-gcc: outside the bounds of array
// tidal-error-clang: dereferenced one-past-the-end pointer
// tidal-error-edg: one position past the end of an array
// tidal-note: Gap review 2026-10-31.
constexpr int f() { int a[2] = {}; a[2] = 1; return 0; }
static_assert(f() == 0);
