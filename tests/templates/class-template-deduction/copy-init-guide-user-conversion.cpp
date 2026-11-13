// tidal-rules: CTAD-17
// tidal-expect: accept
// tidal-expect-clang-18: reject
// tidal-error-clang-18: no viable constructor or deduction guide
// tidal-note: In copy initialization, the user guide W(int) is viable through the conversion function of Conv. The limit to standard sequences of OVL-14 applies to the constructors of W<long> in the initialization that follows, and W(const Conv&) takes the Conv directly. Clang 18 applies the limit to the guide, as the gap reviews of the phase 15 and phase 18 text read the C++23 text (Q-0457).
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct Conv { constexpr operator int() const { return 1; } };
template <class T> struct W { constexpr W(const Conv&) {} };
W(int) -> W<long>;
W w = Conv{};
static_assert(same<decltype(w), W<long>>);
