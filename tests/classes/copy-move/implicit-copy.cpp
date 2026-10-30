// tidal-rules: CPY-4
// tidal-expect: run
struct S { int a = 1; int b = 2; };
int main() { S x; x.a = 5; S y(x); S z; z = x; return y.a == 5 && z.a == 5 && y.b == 2 ? 0 : 1; }
