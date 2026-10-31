// tidal-rules: LSP-1
// tidal-expect: run
extern "C" { struct S { int f() { return 1; } int f(int) { return 2; } }; }
int main() { S s; return s.f() + s.f(0) == 3 ? 0 : 1; }
