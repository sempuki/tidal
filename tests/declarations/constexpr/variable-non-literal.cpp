// tidal-rules: CX-1
// tidal-expect: reject
struct S { ~S() {} };
constexpr S s{};
