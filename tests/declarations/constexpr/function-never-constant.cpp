// tidal-rules: CX-3
// tidal-expect: accept
// tidal-expect-clang-18: reject
// tidal-error-clang: constexpr function never produces a constant expression
// tidal-note: Q-0020. Clang 18 predates the C++23 rule that accepts a constexpr function that can never be constant.
int read() { return 1; }
constexpr int f() { return read(); }
