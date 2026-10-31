// tidal-rules: DCMP-1
// tidal-expect: run
class P { int x; public: P(int v) : x(v) {} bool operator==(const P&) const = default; };
int main() { return P(3) == P(3) ? 0 : 1; }
