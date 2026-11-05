// tidal-rules: ENUM-10
// tidal-expect: accept
// tidal-note: The initializer has the type of an unscoped enumeration with the fixed type short. Inside the list, the new enumerator has that enumeration type, and not short, on every matrix compiler. The gap review of the text phase 8 wrote reads the C++23 text as giving short (documentation/decisions/pending.md, Matrix against review).
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
enum Small : short { one = 1 };
enum Copy { copy = one, check = same<decltype(copy), Small> ? 1 : same<decltype(copy), short> ? 2 : 3 };
static_assert(check == 1);
