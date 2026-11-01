// tidal-rules: NAM-37 PFX-10
// tidal-expect: run
// tidal-note: Member lookup after the dot finds an enumerator, a static data member, a member function, a member of a base, and a qualified member, as C::m does.
struct Base { int b = 4; };
struct Meter : Base {
  enum Color { red = 1 };
  static constexpr int limit = 2;
  int v = 3;
  int get() const { return v; }
};
int main() {
  Meter m;
  if (m.red != 1) return 1;
  if (m.limit != 2) return 2;
  if (m.get() != 3) return 3;
  if (m.b != 4) return 4;
  if (m.Color::red != 1) return 5;
  return m.Meter::v == 3 ? 0 : 6;
}
