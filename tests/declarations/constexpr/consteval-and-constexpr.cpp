// tidal-rules: CX-5
// tidal-expect: reject
// tidal-error-clang: cannot combine with previous .constexpr. declaration specifier
// tidal-error-gcc: both .constexpr. and .consteval. specified
// tidal-error-edg: only one of .constexpr., .consteval., and .constinit.
constexpr consteval int f() { return 1; }
