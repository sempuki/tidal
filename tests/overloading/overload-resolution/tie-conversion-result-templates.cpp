// tidal-rules: OVL-26
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: more than one
// tidal-note: Q-0163. Both conversion function templates give a result that reaches int (*)(). The first result is the identity. The second needs a function pointer conversion, which is worse. Both are templates, so OVL-27 does not apply. EDG finds the two ambiguous.
template <class T> using fp = T (*)();
template <class T> using nfp = T (*)() noexcept;
constexpr int a() { return 1; }
constexpr int b() noexcept { return 2; }
struct Source {
  template <class T> constexpr operator fp<T>() const { return a; }
  template <class T> constexpr operator nfp<T>() const { return b; }
};
constexpr int (*p)() = Source{};
static_assert(p() == 1);
