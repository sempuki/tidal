// tidal-rules: CMP-22
// tidal-expect: run
// tidal-note: Gap review 2026-10-31 (third).
struct A { int x; };
struct B : A {};
struct C : A {};
struct D : B, C {};
int main() {
  int B::* pb = &A::x;
  int C::* pc = &A::x;
  int D::* p1 = pb;
  int D::* p2 = pc;
  int D::* p3 = pb;
  return p1 != p2 && p1 == p3 ? 0 : 1;
}
