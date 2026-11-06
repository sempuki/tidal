// tidal-rules: NAM-21
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: unknown type name
// tidal-note: Gap review 2026-11-05 of the text phase 10 wrote. Q-0415. In a friend declaration of N::f, the name T after the declarator is searched first in N, then by ordinary lookup. GCC finds N::T. Clang does not search N.
namespace N { using T = int; void f(T); }
struct S { friend void N::f(T); };
