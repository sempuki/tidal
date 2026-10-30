// tidal-rules: CONV-3
// tidal-expect: accept
constexpr bool a = 5;
constexpr bool b = 0.0;
constexpr bool c = -0.0;
int x;
bool d = &x;
constexpr bool e = static_cast<int*>(nullptr);
static_assert(a && !b && !c && !e);
