// tidal-rules: TARG-22
// tidal-expect: accept
// tidal-note: The alias template changes its argument, so it is a template of its own on every compiler. Q-0168 concerns an alias that passes its parameters through.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <class T> struct List {};
template <class T> using PtrList = List<T*>;
template <template <class> class C> struct Use {};
static_assert(!same<Use<List>, Use<PtrList>>);
static_assert(same<PtrList<int>, List<int*>>);
