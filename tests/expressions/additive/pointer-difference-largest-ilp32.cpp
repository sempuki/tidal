// tidal-rules: ADD-18 ADD-7
// tidal-expect: accept
// tidal-flags: -m32
static_assert(sizeof(&"" [0] - &"" [0]) == 4);
extern char big[0x7fffffff];
constexpr auto d = &big[0x7fffffff] - &big[0];
static_assert(d == 0x7fffffff);
constexpr auto e = &big[0] - &big[0x7fffffff];
static_assert(e == -0x7fffffff);
