// tidal-rules: NAM-5
// tidal-expect: run
namespace geo { struct P { int v; }; int size(P p) { return p.v; } }
int size(int v) { return v; }
int main() { geo::P p{3}; return size(p) == 3 && size(4) == 4 ? 0 : 1; }
