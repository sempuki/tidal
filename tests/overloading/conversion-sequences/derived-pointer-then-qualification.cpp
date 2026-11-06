// tidal-rules: ICS-26
// tidal-expect: accept
// tidal-note: Decision 0059, item F2 (I3-1), with the round 9 writer probe i3-1-pointer-then-qualification.cpp. Derived* to const Middle* is a derived-to-base conversion and then a qualification adjustment, so Derived* to Middle* is a proper subsequence of it and wins.
struct Base {};
struct Middle : Base {};
struct Derived : Middle {};
constexpr int f(Middle*) { return 1; }
constexpr int f(const Middle*) { return 2; }
constexpr Derived* none = nullptr;
static_assert(f(none) == 1);
