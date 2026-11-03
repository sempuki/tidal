// tidal-rules: UNI-16 UNI-15 UNI-10 STG-23
// tidal-expect: run
// tidal-note: Two enumerations with the same underlying type are layout-compatible, scoped or not, so their members match in the common initial sequence. The read of a Kind object through a Shape glvalue is one that UNI-10 allows, so STG-23 does not make it undefined.
enum class Kind : short { circle, square };
enum Shape : short { round, boxy };
struct Circle { Kind kind; int radius; };
struct Square { Shape kind; int side; };
union Figure { Circle circle; Square square; };
int main() {
  Figure f{.circle = {Kind::square, 2}};
  return f.square.kind == boxy ? 0 : 1;
}
