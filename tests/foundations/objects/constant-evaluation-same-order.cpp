// tidal-rules: OBJ-3
// tidal-expect: accept
struct L { int* out; int id; constexpr L(int* o, int i) : out(o), id(i) {} constexpr ~L() { *out = *out * 10 + id; } };
constexpr int f() { int tr = 0; { L a(&tr, 1); L b(&tr, 2); } return tr; }
static_assert(f() == 21);
