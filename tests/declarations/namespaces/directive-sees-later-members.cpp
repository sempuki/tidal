// tidal-rules: NS-8
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
namespace lib { int first = 1; }
using namespace lib;
namespace lib { int later = 4; }
int main() { return first + later == 5 ? 0 : 1; }
