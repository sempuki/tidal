// tidal-rules: DED-38
// tidal-expect: accept
// tidal-note: T appears in no part of the function type, so id<char> and id<long> both have the type int(int). They are two functions with two addresses.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> int id(int v) { return v; }
static_assert(same<decltype(&id<char>), int (*)(int)>);
static_assert(&id<char> != &id<long>);
