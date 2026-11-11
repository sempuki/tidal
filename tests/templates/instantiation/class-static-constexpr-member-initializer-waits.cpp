// tidal-rules: INST-5
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: has no member
// tidal-note: I6-15. The initializer of a static constexpr member waits for a use of the member. The object of Limit<1> and sizeof do not use the member, so the initializer, which names a member that Tag<1> does not have, is not made. EDG makes the initializer with the class (Q-0231).
template <int K> struct Tag {};
template <int N> struct Limit { static constexpr int value = Tag<N>::missing; };
Limit<1> held;
static_assert(sizeof(Limit<1>) >= 1);
