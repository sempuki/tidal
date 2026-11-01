// tidal-rules: ODR-10
// tidal-expect: run
// tidal-sources: variable-template-differs-across-units.tu.cpp
// tidal-note: first_of has a different initializer in each unit. ODR-10 needs no diagnostic, and the program accepts either value.
template <class T> T first_of = T(1);
int first_elsewhere();
int main() {
  int here = first_of<int>, there = first_elsewhere();
  return (here == 1 || here == 2) && (there == 1 || there == 2) ? 0 : 1;
}
