// tidal-rules: ODR-10
// tidal-expect: run
// tidal-sources: class-template-differs-across-units.tu.cpp
// tidal-note: Pair<int> has one member in this unit and two in the other. ODR-10 needs no diagnostic, and the program accepts each size.
template <class T> struct Pair { T first; };
int size_elsewhere();
int main() {
  int here = sizeof(Pair<int>), there = size_elsewhere();
  return here > 0 && there > 0 ? 0 : 1;
}
