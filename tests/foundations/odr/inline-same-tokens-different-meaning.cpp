// tidal-rules: ODR-10
// tidal-expect: run
// tidal-sources: inline-same-tokens-different-meaning.tu.cpp
static const int factor = 2;
inline int scale(int v) { return v * factor; }
int scale_elsewhere(int v);
int main() {
  int here = scale(10), there = scale_elsewhere(10);
  return (here == 20 || here == 30) && (there == 20 || there == 30) ? 0 : 1;
}
