// tidal-rules: ODR-10
// tidal-expect: run
// tidal-sources: enumeration-differs-across-units.tu.cpp
enum class Level { low, high };
int high_elsewhere();
int main() { return static_cast<int>(Level::high) + high_elsewhere() == 6 ? 0 : 1; }
