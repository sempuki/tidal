// tidal-rules: BF-5
// tidal-expect: run
struct S { unsigned x : 3; };
int main() { S s{}; s.x = 9; return s.x == 1 ? 0 : 1; }
