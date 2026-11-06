// tidal-rules: DED-29
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: no member named .side.
// tidal-note: Q-0420. Decision 0059, item RD, with the round 9 writer probe ded29-address-target.json. The address of the overload set g with a target type deduces T as Circle for the template, whose result type comes from its body. The body names a member that Circle lacks. Clang instantiates the body and rejects, also though the plain g matches the target. GCC and EDG accept and pick the plain function.
struct Circle { double r; };
constexpr int g(const Circle&) { return 0; }
template <class T> constexpr auto g(const T& t) { return t.side; }
int (*pick)(const Circle&) = g;
