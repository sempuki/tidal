// tidal-rules: RET-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
void log_it() {}
auto forward_it() { return log_it(); }
static_assert(same<decltype(forward_it()), void>);
