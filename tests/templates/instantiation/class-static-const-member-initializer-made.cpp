// tidal-rules: INST-5 INST-6
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: no member named 'missing'
// tidal-error-edg: has no member
// tidal-note: I6-15. The initializer in the class of a static const member that is not inline or constexpr is made with the class (Q-0433). The object of Limit<1> makes the initializer, which names a member that Tag<1> does not have. GCC waits for a use of the member.
template <int K> struct Tag {};
template <int N> struct Limit { static const int value = Tag<N>::missing; };
Limit<1> held;
