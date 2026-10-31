// tidal-rules: SB-7
// tidal-expect: reject
// tidal-error-clang: cannot be declared .constexpr
// tidal-error-gcc: structured binding declaration can(not)? be .constexpr
// tidal-error-edg: structured binding cannot be declared .constexpr
constexpr int a[2] = {1, 2};
constexpr auto [x, y] = a;
