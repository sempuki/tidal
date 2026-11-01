// tidal-rules: DEP-24
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: use 'template' keyword to treat 'box' as a dependent template name
// tidal-error-edg: nontype "T::box" is not a template
// tidal-note: The base list names T::box<int> without the template keyword. GCC reads the name as a template, Clang and EDG ask for the keyword (Q-0225).
struct Has { template <class U> struct box {}; };
template <class T> struct Use : T::box<int> {};
Use<Has> u;
