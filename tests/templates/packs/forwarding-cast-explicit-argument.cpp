// tidal-rules: PACK-23
// tidal-expect: accept
// tidal-note: The explicit argument const int& makes T an lvalue reference type. The prvalue 5 binds to x, and the cast is an lvalue because T is an lvalue reference.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> constexpr bool cast_is_lvalue(T&& x) { return same<decltype((static_cast<T&&>(x))), const int&>; }
static_assert(cast_is_lvalue<const int&>(5));
