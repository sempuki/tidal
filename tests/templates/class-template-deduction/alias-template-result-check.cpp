// tidal-rules: CTAD-28 CTAD-24
// tidal-expect: reject
// tidal-error-clang: no viable constructor or deduction guide
// tidal-error-clang-18: requires template arguments; argument deduction only allowed for class templates
// tidal-error-gcc: class template argument deduction failed
// tidal-error-edg: no instance of constructor "Same" matches
// tidal-note: Through the alias, the user guide deduces Pair<int*, long>, which is not Same<int>, so the guide is not viable. Without the alias the same initializer deduces Pair<int*, long>. Clang 18 has no deduction through an alias template (Q-0240).
template <class A, class B> struct Pair { constexpr Pair(A, B) {} };
template <class T> struct Id { using type = T; };
template <class T> using Same = Pair<T*, T*>;
template <class A, class B> Pair(A, B) -> Pair<A, typename Id<B>::type>;
int i;
Pair plain(&i, 1L);
Same s(&i, 1L);
