// tidal-rules: CTAD-21
// tidal-expect: accept
// tidal-note: The copy candidate takes Copy<T> and the guide from the copy constructor takes const Copy<T>&. Ranking and partial ordering tie, neither is a user guide, and neither comes from a constructor template. The copy candidate wins, and the use is not ambiguous.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Copy { constexpr Copy(T) {} constexpr Copy(const Copy&) {} };
constexpr Copy<int> one(1);
Copy two(one);
static_assert(same<decltype(two), Copy<int>>);
