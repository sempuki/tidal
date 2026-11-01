// tidal-rules: DEP-24
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: use 'template' keyword to treat 'box' as a dependent template name
// tidal-error-edg: may not have a template argument list
// tidal-note: GCC reads the name after typename T:: as a template when an argument list follows. Clang and EDG ask for the template keyword (Q-0225).
struct Has { template <class U> struct box { U v; }; };
template <class T> int run() { typename T::box<int> b{1}; return b.v; }
int x = run<Has>();
