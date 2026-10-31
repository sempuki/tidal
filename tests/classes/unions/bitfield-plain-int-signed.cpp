// tidal-rules: BF-5
// tidal-expect: run
struct S { int x : 3; };
int main() { S s{}; s.x = -1; return s.x == -1 ? 0 : 1; }
