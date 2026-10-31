// tidal-rules: MPTR-6
// tidal-expect: reject
struct B { int x = 0; };
struct D : B { int y = 5; };
constexpr int read() { B b; int B::* pm = static_cast<int B::*>(&D::y); return b.*pm; }
static_assert(read() == 5);
