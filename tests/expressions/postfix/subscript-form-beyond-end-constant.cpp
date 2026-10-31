// tidal-rules: PFX-4
// tidal-expect: reject
// tidal-error-gcc: outside the bounds of array
// tidal-error-clang: cannot refer to element .* of array of
// tidal-error-edg: cannot access position .* in array of
// tidal-note: Gap review 2026-10-31.
constexpr int f() { int a[2] = {}; int* p = &a[3]; return p != nullptr; }
static_assert(f() == 1);
