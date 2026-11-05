// tidal-rules: ORD-6
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: more than one operator
// tidal-note: The const member gets the object parameter const Grid&, the same type as the other template's first parameter. That pair ties, and T* is more specialized than T. EDG finds the call ambiguous (Q-0398).
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct Ptr {}; struct Any {};
struct Grid { template <class T> Ptr operator+(T*) const; };
template <class T> Any operator+(const Grid&, T);
const Grid grid;
int* p = nullptr;
static_assert(same<decltype(grid + p), Ptr>);
