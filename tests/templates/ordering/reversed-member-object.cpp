// tidal-rules: ORD-7 ORD-6
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-expect-edg: reject
// tidal-error-gcc: static assert
// tidal-error-edg: static assertion failed
// tidal-note: For p == x the member is a swapped candidate. Its object parameter is added first and then the two are reversed, which gives (T* const&, const X&). That is more specialized than the non-member's (const T&, const X&). GCC and EDG choose the non-member (Q-0218).
struct X { template <class T> constexpr bool operator==(T* const&) const { return true; } };
template <class T> constexpr bool operator==(const T&, const X&) { return false; }
constexpr X x;
constexpr int* p = nullptr;
static_assert(p == x);
