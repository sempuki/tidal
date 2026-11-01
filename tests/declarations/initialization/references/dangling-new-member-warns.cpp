// tidal-rules: REF-15
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: will be destroyed at the end of the full-expression
// tidal-flags: -Wall -Werror
// tidal-note: Q-0180. GCC does not warn about a temporary bound to a reference member of an object that new makes.
struct Holder { const int& n; };
Holder* make() { return new Holder{1}; }
