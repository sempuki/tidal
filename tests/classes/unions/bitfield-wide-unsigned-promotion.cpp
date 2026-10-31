// tidal-rules: BF-6
// tidal-expect: run
struct S { unsigned long long x : 40; };
int main() { S s{}; s.x = 2; return s.x - 5 > 0 ? 0 : 1; }
