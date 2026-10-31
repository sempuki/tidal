// tidal-rules: ADD-10
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: cannot refer to element -1 of array
// tidal-error-edg: cannot set pointer before the first array element
// tidal-note: Q-0031. GCC accepts forming a pointer outside the array during constant evaluation; Clang and EDG reject it.
constexpr int a[3] = {1, 2, 3};
constexpr const int* p = a - 1;
