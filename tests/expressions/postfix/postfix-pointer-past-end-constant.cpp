// tidal-rules: PFX-18
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: cannot refer to element .* of array of
// tidal-error-edg: cannot access position .* in array of
// tidal-note: Q-0031. GCC accepts forming a pointer past the end of an array during constant evaluation.
constexpr int f() { int a[2] = {}; int* p = a + 2; p++; return 0; }
static_assert(f() == 0);
