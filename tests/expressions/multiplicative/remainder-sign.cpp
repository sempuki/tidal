// tidal-rules: MUL-7
// tidal-expect: accept
static_assert(7 % 2 == 1);
static_assert(-7 % 2 == -1);
static_assert(7 % -2 == 1);
static_assert(-7 % -2 == -1);
constexpr bool identity(int a, int b) { return (a / b) * b + a % b == a; }
static_assert(identity(17, 5) && identity(-17, 5) && identity(17, -5) && identity(-17, -5));
