// tidal-rules: EXS-2
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
void f() noexcept;
static_assert(same<decltype(f), void() noexcept>);
static_assert(same<decltype(&f), void (*)() noexcept>);
