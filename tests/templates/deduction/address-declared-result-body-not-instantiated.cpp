// tidal-rules: DED-29 DED-24
// tidal-expect: accept
// tidal-note: Decision 0059, item RD, with the round 9 writer probe ded29-address-target.json. When the template's result type is declared, or comes from a trailing decltype that fails for Circle, the address with a target type needs no body, and every compiler picks the plain function.
struct Circle { double r; };
constexpr int g(const Circle&) { return 0; }
template <class T> constexpr double g(const T& t) { return t.side; }
constexpr int h(const Circle&) { return 0; }
template <class T> constexpr auto h(const T& t) -> decltype(t.side) { return t.side; }
int (*pick_g)(const Circle&) = g;
int (*pick_h)(const Circle&) = h;
