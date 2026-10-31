// tidal-rules: CINIT-2
// tidal-expect: reject
// tidal-error-clang: cannot combine with previous .constexpr. declaration specifier
// tidal-error-gcc: at most one of the .constinit. and .constexpr. specifiers
// tidal-error-edg: only one of .constexpr., .consteval., and .constinit.
constexpr constinit int x = 1;
