// tidal-rules: CTAD-2
// tidal-expect: reject
// tidal-error-clang: requires an initializer
// tidal-expect-gcc: accept
// tidal-error-edg: argument list for class template
// tidal-note: A static data member whose type is a class template name with no argument list needs an initializer, inline or not. GCC deduces Box<int> from the guide with no parameters (Q-0439).
template <class T = int> struct Box { T v; };
struct Holder { static Box shared; static inline Box local; };
