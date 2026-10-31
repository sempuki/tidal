// tidal-rules: VOL-2
// tidal-expect: run
struct S { int f() { return 1; } int f() volatile { return 2; } };
int main() { S a; volatile S b; return a.f() == 1 && b.f() == 2 ? 0 : 1; }
