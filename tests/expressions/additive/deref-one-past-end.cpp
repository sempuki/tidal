// tidal-rules: ADD-10
// tidal-expect: reject
// tidal-error-gcc: array subscript value .3. is outside the bounds
constexpr int a[3] = {1, 2, 3};
constexpr int v = *(a + 3);
