// tidal-rules: BF-10
// tidal-expect: run
struct S { int x : 3; };
int main() { S s{}; int v = 5; s.x = v; return s.x == -3 ? 0 : 1; }
