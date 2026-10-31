// tidal-rules: CVF-2
// tidal-expect: run
int seven() { return 7; }
using Ref = int (&)();
struct S { operator Ref() const { return seven; } };
int main() { S s; return s() == 7 ? 0 : 1; }
