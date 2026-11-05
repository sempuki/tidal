// tidal-rules: DED-24
// tidal-expect: accept
// tidal-note: An explicit argument of -1 makes an array bound below one, which is not a valid type (DCL-19). Each template with the array drops out, and the other overload is chosen.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct One {}; struct Two {};
template <int N> One by_reference(char (&)[N]);
template <int N> Two by_reference(...);
template <int N> One by_pointer(char (*)[N]);
template <int N> Two by_pointer(int);
static_assert(same<decltype(by_reference<-1>(0)), Two>);
static_assert(same<decltype(by_reference<1>(0)), Two>);
static_assert(same<decltype(by_pointer<-1>(0)), Two>);
