// tidal-rules: DED-20
// tidal-expect: accept
// tidal-note: Each element of the pack deduces as one parameter would, with decay and forwarding.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
template <class... Ts> Tag<void(Ts...)> all(Ts...);
template <class... Ts> Tag<void(Ts...)> refs(Ts&&...);
template <class... Ts> struct List {};
template <class... Ts> Tag<void(Ts...)> unpack(List<Ts...>);
int arr[3];
int n = 0;
static_assert(same<decltype(all(1, 'c', arr)), Tag<void(int, char, int*)>>);
static_assert(same<decltype(all()), Tag<void()>>);
static_assert(same<decltype(refs(n, 1)), Tag<void(int&, int)>>);
static_assert(same<decltype(unpack(List<int, char>{})), Tag<void(int, char)>>);
