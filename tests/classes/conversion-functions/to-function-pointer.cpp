// tidal-rules: CVF-1
// tidal-expect: run
int seven() { return 7; }
using Fp = int (*)();
struct S { operator Fp() const { return seven; } };
int main() { S s; return s() == 7 ? 0 : 1; }
