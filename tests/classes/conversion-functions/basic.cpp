// tidal-rules: CVF-1
// tidal-expect: run
struct Meters { int v; operator int() const { return v; } };
int main() { Meters m{4}; int x = m; return x == 4 ? 0 : 1; }
