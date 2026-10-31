// tidal-rules: INL-4
// tidal-expect: run
struct S { static const int n; };
inline const int S::n = 9;
int main() { return S::n == 9 ? 0 : 1; }
