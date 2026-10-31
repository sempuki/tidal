// tidal-rules: DEFARG-2
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-note: Q-0111. Clang does not see a default argument that a later namespace declaration adds, through an earlier using declaration. GCC and EDG accept the call.
// tidal-note: Gap review 2026-10-31.
namespace n { void f(int); }
using n::f;
namespace n { void f(int = 3); }
void g() { f(); }
