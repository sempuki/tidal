// tidal-rules: DED-38 DED-24
// tidal-expect: accept
// tidal-note: The function types of keep<int> and keep<const int> are both void(int) after adjustment. They are still two functions with two addresses.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> void keep(T) {}
static_assert(same<decltype(&keep<const int>), void (*)(int)>);
static_assert(&keep<int> != &keep<const int>);
