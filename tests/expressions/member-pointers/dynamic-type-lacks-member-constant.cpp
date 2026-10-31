// tidal-rules: MPTR-6
// tidal-expect: reject
// tidal-error-gcc: \(\(sizetype\)pm\)\). is not a constant expression
// tidal-error-edg: invalid access to member .D::y. in object of complete type .B
struct B { int x = 0; };
struct D : B { int y = 5; };
constexpr int read() { B b; int B::* pm = static_cast<int B::*>(&D::y); return b.*pm; }
static_assert(read() == 5);
