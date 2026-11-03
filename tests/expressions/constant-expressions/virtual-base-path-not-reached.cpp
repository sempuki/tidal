// tidal-rules: CE-2 DER-12
// tidal-expect: run
// tidal-note: Write report W9. A constexpr function can create an object of a class with a virtual base on a path that constant evaluation does not reach.
struct Base { int n = 3; };
struct Left : virtual Base {};
constexpr int make(bool build) {
  if (build) {
    Left x;
    return x.n;
  }
  return 0;
}
static_assert(make(false) == 0);
int main() { return make(true) == 3 ? 0 : 1; }
