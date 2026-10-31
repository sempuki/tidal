// tidal-rules: OPR-3
// tidal-expect: run
struct Arr { int d[3] = {1, 2, 3}; int& operator[](int i) { return d[i]; } };
int main() { Arr a; a[1] = 9; return a[1] == 9 ? 0 : 1; }
