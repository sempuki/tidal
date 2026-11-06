// tidal-rules: DED-29
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: no member named .side.
// tidal-note: Q-0420. Decision 0059, item RD, with the round 9 writer probe ded29-address-target.json. The same overload set as address-deduced-result-body-instantiated.cpp, with the target type given by a static_cast. Clang rejects; GCC and EDG accept.
struct Circle { double r; };
constexpr int g(const Circle&) { return 0; }
template <class T> constexpr auto g(const T& t) { return t.side; }
auto pick = static_cast<int (*)(const Circle&)>(g);
