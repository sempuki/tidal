// tidal-rules: DEFARG-2
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
namespace n { void f(int); }
using n::f;
namespace n { void f(int = 3); }
void g() { f(); }
