// tidal-rules: NS-7
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
namespace a { int x = 1; }
namespace b { int y = 2; }
using a::x, b::y;
int main() { return x + y == 3 ? 0 : 1; }
