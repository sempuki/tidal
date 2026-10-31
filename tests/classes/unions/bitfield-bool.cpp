// tidal-rules: BF-1
// tidal-expect: run
struct S { bool on : 1; };
int main() { S s{}; s.on = true; return s.on ? 0 : 1; }
