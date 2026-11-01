// tidal-rules: TARG-19
// tidal-expect: accept
// tidal-note: Equal values, an alias of a type, a default written out, and an alias template name the same specialization.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <class T, int N = 2> struct Box {};
template <class T> struct List {};
template <template <class> class C> struct Use {};
template <class T> using ListAlias = List<T>;
using Integer = int;
static_assert(same<Box<int, 1 + 1>, Box<int, 2>>);
static_assert(same<Box<Integer>, Box<int, 2>>);
static_assert(!same<Box<int, 3>, Box<int, 2>>);
static_assert(!same<Box<const int>, Box<int>>);
static_assert(same<Use<List>, Use<List>>);
