// tidal-rules: DED-8 VC-2
// tidal-expect: accept
// tidal-note: I5-6, I5 program p04-const-prvalue-forwarding.cpp. A call of a function that returns const int is an int prvalue, because a prvalue of a type that is not a class has no cv-qualifiers (VC-2). So T is int.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
template <class T> Tag<T> fwd(T&&);
const int give();
const int* where();
static_assert(same<decltype(fwd(give())), Tag<int>>);
static_assert(same<decltype(fwd(where())), Tag<const int*>>);
