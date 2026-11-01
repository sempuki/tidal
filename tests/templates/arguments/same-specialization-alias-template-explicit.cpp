// tidal-rules: TARG-22
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-expect-edg: reject
// tidal-note: Use has an explicit specialization for List. Clang treats ListAlias as a template of its own, so Use<ListAlias> uses the primary template. GCC and EDG treat it as List, and the assertion fails. TARG-22 allows both (Q-0168).
template <class T> struct List {};
template <class T> using ListAlias = List<T>;
template <template <class> class C> struct Use { static constexpr int value = 0; };
template <> struct Use<List> { static constexpr int value = 1; };
static_assert(Use<ListAlias>::value == 0);
