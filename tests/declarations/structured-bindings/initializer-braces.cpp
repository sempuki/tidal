// tidal-rules: SB-7
// tidal-expect: accept
// tidal-expect-clang-18: reject
// tidal-error-clang: array element of type .int. with an lvalue of type .int\[2\]
// tidal-note: Q-0015. Clang 18 rejects a braced array initializer for a structured binding; Clang 22 accepts.
int a[2] = {1, 2};
auto [r, s]{a};
