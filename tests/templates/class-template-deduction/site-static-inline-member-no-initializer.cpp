// tidal-rules: CTAD-2
// tidal-expect: reject
// tidal-error-clang: requires an initializer
// tidal-expect-gcc: accept
// tidal-error-edg: argument list for class template
// tidal-note: Gap review RV8-B-4. An inline static data member whose type is a class template name with no argument list, and with no initializer, does not deduce. Clang and EDG reject the member. GCC deduces Box<int> from the guide with no parameters, as the review reads the reference (Q-0439).
template <class T = int> struct Box { T v; };
struct Holder { static inline Box local; };
