// tidal-rules: ODR-10
// tidal-expect: run
// tidal-sources: class-differs-across-units.tu.cpp
struct Record { int id; };
int size_elsewhere();
int main() {
  int here = sizeof(Record), there = size_elsewhere();
  return here > 0 && there > 0 ? 0 : 1;
}
