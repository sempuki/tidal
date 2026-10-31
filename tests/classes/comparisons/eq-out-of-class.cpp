// tidal-rules: DCMP-1
// tidal-expect: run
struct P { int x; bool operator==(const P&) const; };
bool P::operator==(const P&) const = default;
int main() { return P{1} == P{1} ? 0 : 1; }
