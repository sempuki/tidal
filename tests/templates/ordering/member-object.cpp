// tidal-rules: ORD-6
// tidal-expect: accept
// tidal-note: The member template gets the object parameter X&, which the non-member matches with T&. The member is then more specialized. A member with && gets X&&, which the non-member matches with T&&.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
struct One {}; struct Two {};
struct X { template <class T> One operator+(T*); };
template <class T, class U> Two operator+(T&, U);
X x;
int* p = nullptr;
static_assert(same<decltype(x + p), One>);
struct Y { template <class T> One operator+(T*) &&; };
template <class T, class U> Two operator+(T&&, U);
static_assert(same<decltype(Y{} + p), One>);
