// tidal-rules: DER-2
// tidal-expect: run
struct A { int v = 0; };
struct B : A {};
struct C : A {};
struct D : B, C {};
int main() { D d; static_cast<B&>(d).v = 1; static_cast<C&>(d).v = 2;
  return static_cast<B&>(d).v == 1 && static_cast<C&>(d).v == 2 ? 0 : 1; }
