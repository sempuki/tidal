// tidal-rules: CX-1
// tidal-expect: reject
// tidal-error-clang: constexpr variable cannot have non-literal type
// tidal-error-gcc: constexpr. variable .s. is not literal
// tidal-error-edg: constexpr variable must have a literal type
struct S { ~S() {} };
constexpr S s{};
