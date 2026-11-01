// tidal-rules: DEP-26
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: template argument list is expected|does not refer to a template
// tidal-error-edg: argument list for template
// tidal-note: The template keyword comes before a data member with no argument list. Clang and EDG reject the instantiation, GCC accepts it (Q-0226).
struct Has { int field = 0; };
template <class T> int run(T object) { return object.template field; }
int x = run(Has{});
