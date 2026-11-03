// tidal-rules: THR-10
// tidal-expect: run
// tidal-note: A constexpr constructor that throws, called at run time.
struct Positive {
  int v;
  constexpr Positive(int x) : v(x) {
    if (x < 0) throw x;
  }
};
int main(int argc, char**) {
  try {
    Positive p(-argc);
    return p.v;
  } catch (int e) {
    return e == -argc ? 0 : 1;
  }
}
