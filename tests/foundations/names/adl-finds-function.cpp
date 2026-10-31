// tidal-rules: NAM-5
// tidal-expect: run
namespace geo { struct P { int v; }; int twice(P p) { return p.v * 2; } }
int main() { geo::P p{3}; return twice(p) == 6 ? 0 : 1; }
