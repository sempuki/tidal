// tidal-rules: DED-29 OVL-27
// tidal-expect: run
// tidal-note: Front-end finding G152 (reports/clean-room/rust-frontend/report-22.md), roadmap round 8 programmer reduction overload_set_generic_body_invalid.cpp. The generic call operator's body reads s.base, which Circle lacks. For area(Circle{1}) the plain call operator wins, and the losing candidate's body is never instantiated.
struct Circle { int r; };
struct Rect { int base; };
template <class... F> struct overloaded : F... { using F::operator()...; };
int main() {
  overloaded area{
    [](const Circle& c) { return 3 * c.r * c.r; },
    [](const auto& s) { return s.base; },
  };
  return area(Circle{1}) + area(Rect{2}) == 5 ? 0 : 1;
}
