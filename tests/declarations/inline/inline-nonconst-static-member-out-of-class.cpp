// tidal-rules: INL-4
// tidal-expect: run
struct S { static int n; };
inline int S::n = 5;
int main() { return S::n == 5 ? 0 : 1; }
