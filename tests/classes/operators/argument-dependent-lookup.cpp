// tidal-rules: OPR-6
// tidal-expect: run
namespace geo { struct P { int x; }; P operator+(P a, P b) { return {a.x + b.x}; } }
int main() { geo::P a{1}, b{2}; return (a + b).x == 3 ? 0 : 1; }
