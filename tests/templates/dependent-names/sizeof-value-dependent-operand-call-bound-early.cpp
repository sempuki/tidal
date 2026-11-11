// tidal-rules: DEP-4 DEP-5
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: static assertion failed
// tidal-note: I6-6. sizeof(N) and noexcept(N + 1) are not value-dependent, although N is. So Slot<sizeof(N)> and Slot<noexcept(N + 1)> are types that do not depend on N, each call has no dependent argument, and each binds at the definition to the function with the parameter long. The later functions with the parameter int are not seen. EDG treats both operands as value-dependent and finds the later functions at the instantiation (Q-0432).
namespace lib {
template <decltype(sizeof 0) K> struct Slot {};
constexpr int pick(Slot<sizeof(int)>, long) { return 1; }
constexpr int pick(Slot<1>, long) { return 1; }
}
template <int N> constexpr int by_size() { return pick(lib::Slot<sizeof(N)>{}, 0); }
template <int N> constexpr int by_noexcept() { return pick(lib::Slot<noexcept(N + 1)>{}, 0); }
namespace lib {
constexpr int pick(Slot<sizeof(int)>, int) { return 2; }
constexpr int pick(Slot<1>, int) { return 2; }
}
static_assert(by_size<4>() == 1);
static_assert(by_noexcept<4>() == 1);
