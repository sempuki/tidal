// tidal-rules: UNI-9
// tidal-expect: accept
// tidal-note: Gap review RV6-A-17. A static_assert declaration is no member, and an anonymous union can contain one. Every compiler accepts it.
struct Cell {
  union { int whole; float real; static_assert(sizeof(int) == sizeof(float)); };
  Cell() : whole(1) {}
};
int main() { Cell c; return c.whole == 1 ? 0 : 1; }
