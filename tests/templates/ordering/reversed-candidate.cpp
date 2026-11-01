// tidal-rules: ORD-7
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: static assert
// tidal-note: The swapped candidate is ordered with its parameters reversed, as (const X&, T* const&), which is more specialized than (const X&, const T&). Without the reversal neither is more specialized, and the written candidate would win. GCC does not reverse the parameters and chooses the written candidate (Q-0211).
struct X {};
template <class T> constexpr bool operator==(const X&, const T&) { return false; }
template <class T> constexpr bool operator==(T* const&, const X&) { return true; }
constexpr X x;
constexpr int* p = nullptr;
static_assert(x == p);
