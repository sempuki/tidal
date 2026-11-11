// Planted bug: a value outside the range of an enumeration with no fixed type.
enum Level { low, mid, high };
int main() { volatile int n = 9; Level l = static_cast<Level>(n); return l == high ? 1 : 0; }
