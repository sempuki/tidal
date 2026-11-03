// tidal-rules: CTAD-27 CTAD-9
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: class template argument deduction failed
// tidal-note: The trailing pack of bases takes both elements of the list in parentheses. GCC forms no aggregate candidate from parentheses for a class with a pack of bases, and accepts the same list in braces (Q-0352).
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct Left {};
struct Right {};
template <class... F> struct Both : F... {};
Both both(Left{}, Right{});
static_assert(same<decltype(both), Both<Left, Right>>);
