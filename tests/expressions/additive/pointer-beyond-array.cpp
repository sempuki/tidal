// tidal-rules: ADD-10
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: cannot refer to element .* of array of
// tidal-error-edg: cannot access position .* in array of
// tidal-note: Q-0031. GCC accepts forming a pointer outside the array during constant evaluation; Clang and EDG reject it.
constexpr int a[3] = {1, 2, 3};
constexpr const int* p = a + 4;
