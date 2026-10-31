// tidal-rules: OVL-6
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: call of .\(const Thing\) \(int\). is ambiguous
// tidal-note: Q-0144. The conversion function is not const, so a const object gets no surrogate. The call operator is the only viable candidate. GCC keeps the surrogate, which ties with the call operator.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
using Fn = One (*)(int);
struct Thing {
  operator Fn();
  Two operator()(long) const;
};
const Thing c{};
static_assert(same<decltype(c(1)), Two>);
