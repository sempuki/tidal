// tidal-rules: PACK-13
// tidal-expect: run
// tidal-note: Roadmap round 8, programmer reduction variable_paren_pack.cpp (reports/clean-room/roadmap-8/programmer.md, section 1.5). A pack expands in the expression list of a parenthesized initializer of a variable; each copy of the pattern is one argument of the constructor.
struct P { int x, y; P(int a, int b) : x(a), y(b) {} };
template <class T, class... A> int make(A... a) { T x(a...); return x.x + x.y; }
int main() { return make<P>(1, 2) == 3 ? 0 : 1; }
