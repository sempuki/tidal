// tidal-rules: ODR-10
// tidal-expect: run
// tidal-sources: function-template-same-tokens-different-meaning.tu.cpp
// tidal-note: factor names a different constant in each unit. ODR-10 needs no diagnostic, and the program accepts the result of either definition.
static const int factor = 2;
template <class T> T scaled(T v) { return v * factor; }
int scaled_elsewhere(int v);
int main() {
  int here = scaled(10), there = scaled_elsewhere(10);
  return (here == 20 || here == 30) && (there == 20 || there == 30) ? 0 : 1;
}
