// tidal-rules: ODR-10
// tidal-expect: run
// tidal-sources: inline-variable-differs-across-units.tu.cpp
inline int limit = 1;
int limit_elsewhere();
int main() {
  int here = limit, there = limit_elsewhere();
  return (here == 1 || here == 2) && (there == 1 || there == 2) ? 0 : 1;
}
