// tidal-rules: OBJ-7
// tidal-expect: run
int main() { int x = 0; int* p = &x; x = 5; int* q = &x; return p == q && *p == 5 ? 0 : 1; }
