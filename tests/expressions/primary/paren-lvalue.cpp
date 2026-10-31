// tidal-rules: PRIM-14
// tidal-expect: run
int main() { int x = 0; (x) = 4; ((x)) += 1; return x == 5 ? 0 : 1; }
