// tidal-rules: DCMP-1
// tidal-expect: run
struct P { int x; int y; bool operator==(const P&) const = default; };
int main() { P a{1, 2}, b{1, 2}, c{1, 3}; return a == b && !(a == c) ? 0 : 1; }
