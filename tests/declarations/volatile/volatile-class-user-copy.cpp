// tidal-rules: VOL-4
// tidal-expect: run
struct S { int v; S(int x) : v(x) {} S(const volatile S& o) : v(o.v) {} };
volatile S a(3);
int main() { S b = a; return b.v == 3 ? 0 : 1; }
