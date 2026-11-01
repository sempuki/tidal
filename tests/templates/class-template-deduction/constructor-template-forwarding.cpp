// tidal-rules: CTAD-8
// tidal-expect: accept
// tidal-note: U&& names a parameter of the constructor template, so it is a forwarding reference and takes the lvalue i. T&& names a class parameter and takes the rvalue 1.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Hold { template <class U> constexpr Hold(T&&, U&&) {} };
int i;
Hold h(1, i);
static_assert(same<decltype(h), Hold<int>>);
