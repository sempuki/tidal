// tidal-rules: DEP-26
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: a template argument list is expected after a name prefixed by the template keyword|expected an identifier or template-id
// tidal-note: The keyword template comes before a class template with no argument list, and the arguments are deduced. GCC and EDG accept the program, Clang rejects it (Q-0226).
struct Has { template <class U> struct box { box(U) {} }; };
template <class T> void make() { typename T::template box b(1); }
template void make<Has>();
