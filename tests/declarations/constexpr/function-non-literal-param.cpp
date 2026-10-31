// tidal-rules: CX-3
// tidal-expect: accept
// tidal-expect-clang-18: reject
// tidal-error-clang: parameter type .S. is not a literal type
// tidal-note: Q-0020. Clang 18 predates the C++23 rule that allows non-literal parameter types in constexpr functions.
struct S { ~S() {} };
constexpr int f(S) { return 1; }
