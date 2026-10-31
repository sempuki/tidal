// tidal-rules: CVF-1
// tidal-expect: run
struct S { int v = 4; operator int(this const S& self) { return self.v; } };
int main() { int x = S{}; return x == 4 ? 0 : 1; }
