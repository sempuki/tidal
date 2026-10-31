// tidal-rules: ADD-10
// tidal-expect: reject
// tidal-error-gcc: array subscript value .3. is outside the bounds
// tidal-error-clang: dereferenced one-past-the-end pointer
// tidal-error-edg: one position past the end of an array
constexpr int a[3] = {1, 2, 3};
constexpr int v = *(a + 3);
