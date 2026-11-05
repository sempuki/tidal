// tidal-rules: ORD-13 ORD-6
// tidal-expect: accept
// tidal-expect-clang-18: reject
// tidal-error-clang-18: is ambiguous
// tidal-note: Each member gets the object parameter Item&, so its parameter list (Item&, T) is the same as the other template's, and the heads are the same. The constraint then decides, for the member in the first class and for the other template in the second. Without the constraint the call is ambiguous (member-object-lvalue-tie.cpp). Clang 18 finds both calls ambiguous.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> concept Small = sizeof(T) < 8;
struct Member {}; struct Free {};
struct Item { template <class T> requires Small<T> Member operator-(T); };
template <class T> Free operator-(Item&, T);
Item item;
static_assert(same<decltype(item - 2), Member>);
struct Cell { template <class T> Member operator-(T); };
template <class T> requires Small<T> Free operator-(Cell&, T);
Cell cell;
static_assert(same<decltype(cell - 2), Free>);
