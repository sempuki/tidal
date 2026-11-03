// tidal-rules: DER-8 DER-20
// tidal-expect: run
// tidal-note: The private constructor of B is used inside a member of D, and D is a friend of B, so the inherited constructor is accessible there.
class B { B(int v) : value(v) {} public: int value = 0; friend struct D; };
struct D : B {
  using B::B;
  static D make() { return D(4); }
};
int main() { return D::make().value == 4 ? 0 : 1; }
