// tidal-rules: REF-15
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: will be destroyed at the end of the full-expression
// tidal-flags: -Werror
// tidal-note: Q-0180. Clang gives this warning by default (-Wdangling-field). GCC does not warn.
struct Holder { const int& n; };
Holder* make() { return new Holder{1}; }
