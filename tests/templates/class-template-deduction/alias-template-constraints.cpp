// tidal-rules: CTAD-28 CTAD-24
// tidal-expect: reject
// tidal-error-clang: no viable constructor or deduction guide
// tidal-error-clang-18: requires template arguments; argument deduction only allowed for class templates
// tidal-error-gcc: class template argument deduction failed
// tidal-error-edg: no instance of constructor "Indexed" matches
// tidal-note: The alias Indexed constrains T by Offset, and double does not satisfy it, so the guides of the alias give no candidate for pointers to double. The alias Plain has no constraint and accepts them.
template <class T> concept Offset = requires (T t, T* p) { p + t; };
template <class A, class B> struct Pair { constexpr Pair(A, B) {} };
template <class T> using Plain = Pair<T*, T*>;
template <Offset T> using Indexed = Pair<T*, T*>;
double d;
int i;
Plain p(&d, &d);
Indexed q(&i, &i);
Indexed r(&d, &d);
