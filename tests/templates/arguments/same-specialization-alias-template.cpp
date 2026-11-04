// tidal-rules: TARG-22
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-expect-edg: reject
// tidal-note: TARG-22 makes the alias template a template of its own, so the two types differ, as Clang finds. GCC and EDG treat it as the class template, and the assertion fails (Q-0168).
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <class T> struct List {};
template <class T> using ListAlias = List<T>;
template <template <class> class C> struct Use {};
static_assert(!same<Use<List>, Use<ListAlias>>);
