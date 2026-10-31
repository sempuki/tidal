// tidal-rules: DTY-3
// tidal-expect: run
int f() { return 3; }
int main() { decltype(f()) v = 4; decltype(v)* p = &v; return *p == 4 ? 0 : 1; }
