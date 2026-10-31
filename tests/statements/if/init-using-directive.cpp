// tidal-rules: IF-7
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31 (third).
namespace n { int x; }
void f(int v) { if (using namespace n; v) {} }
