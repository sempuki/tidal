// tidal-rules: IF-21
// tidal-expect: reject
// tidal-error-gcc: expected primary-expression before .using
// tidal-note: Gap review 2026-10-31 (third).
namespace n { int x; }
void f(int v) { if (using n::x; v) {} }
