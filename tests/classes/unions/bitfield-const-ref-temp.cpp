// tidal-rules: BF-4
// tidal-expect: run
struct S { int x : 4; };
int main() { S s{}; s.x = 5; const int& r = s.x; s.x = 2; return r == 5 ? 0 : 1; }
