// tidal-rules: PRIM-14
// tidal-expect: run
int f() { return 3; }
int main() { return (f)() == 3 ? 0 : 1; }
