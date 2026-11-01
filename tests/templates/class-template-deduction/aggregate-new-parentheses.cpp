// tidal-rules: CTAD-9
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: class template argument deduction failed
// tidal-note: A parenthesized list in a new expression forms the aggregate candidate. GCC forms the candidate for a variable and a functional cast, and not in a new expression (Q-0244).
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Agg { T v; };
auto p = new Agg(1);
static_assert(same<decltype(p), Agg<int>*>);
