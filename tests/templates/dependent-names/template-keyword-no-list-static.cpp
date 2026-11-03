// tidal-rules: DEP-26
// tidal-expect: reject
// tidal-error-clang: a template argument list is expected after a name prefixed by the template keyword|does not refer to a template
// tidal-error-gcc: is not a template
// tidal-error-edg: argument list for template
// tidal-note: The keyword template comes after T:: before a static data member, with no argument list. Every compiler rejects the instantiation. GCC accepts the same keyword after a dot before a data member (Q-0226).
struct Has { static int field; };
int Has::field = 0;
template <class T> int run() { return T::template field; }
int x = run<Has>();
