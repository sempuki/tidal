// tidal-rules: ODR-14
// tidal-expect: run
// tidal-sources: explicit-specialization-in-one-unit.tu.cpp
// tidal-note: Only this unit declares the explicit specialization measure<int>. ODR-14 needs no diagnostic, and the program accepts the result of either definition.
template <class T> int measure(T) { return 4; }
template <> int measure(int) { return 99; }
int measure_elsewhere();
int main() {
  int here = measure(1), there = measure_elsewhere();
  return (here == 4 || here == 99) && (there == 4 || there == 99) ? 0 : 1;
}
