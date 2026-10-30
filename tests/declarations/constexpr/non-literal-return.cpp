// tidal-rules: CX-3
// tidal-expect: accept
// tidal-expect-clang-18: reject
// tidal-note: Gap review 2026-10-30. Q-0020. Clang 18 predates the C++23 rule that allows non-literal return types.
struct S { ~S() {} };
constexpr S make() { return S{}; }
