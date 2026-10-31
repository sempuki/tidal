// tidal-rules: CVF-1
// tidal-expect: run
struct S { operator long() const; };
S::operator long() const { return 9; }
int main() { long x = S{}; return x == 9 ? 0 : 1; }
