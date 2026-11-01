// tidal-rules: REF-6 REF-9
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: cannot bind to an initializer list temporary
// tidal-error-edg: initial value of reference to non-const must be an lvalue
// tidal-note: Q-0157. Source is not int, so REF-14 makes a temporary int from {source}, and int& cannot bind it. GCC binds the reference through operator int&.
int target = 1;
struct Source { operator int&() { return target; } };
Source source;
int& alias{source};
