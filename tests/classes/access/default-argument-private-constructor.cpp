// tidal-rules: ACC-16
// tidal-expect: run
// tidal-note: The default argument of a constructor names a private static data member.
class C {
  static constexpr int base = 7;
public:
  int v;
  C(int x = base) : v(x) {}
};
int main() {
  C c;
  return c.v == 7 ? 0 : 1;
}
