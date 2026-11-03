// Probe: a member function pointer cast to the base and applied to a base object (MPTR-6).
struct M { int acc = 0; virtual ~M() = default; };
struct D : M { int extra = 5; void reset() { extra = 0; } };
using Op = void (M::*)();
int main() { M m; Op r = static_cast<Op>(&D::reset); (m.*r)(); return 0; }
