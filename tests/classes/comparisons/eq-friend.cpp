// tidal-rules: DCMP-1
// tidal-expect: run
class P { int x = 0; public: P(int v) : x(v) {} friend bool operator==(const P&, const P&) = default; };
int main() { return P(1) == P(1) && !(P(1) == P(2)) ? 0 : 1; }
