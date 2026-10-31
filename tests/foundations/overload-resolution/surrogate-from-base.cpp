// tidal-rules: OVL-6
// tidal-expect: accept
// tidal-note: A conversion function of a base gives a surrogate. A conversion to the same type in the derived class hides the base one, so one surrogate remains.
using Fn = int (*)(int);
constexpr int first(int) { return 1; }
constexpr int second(int) { return 2; }
constexpr int third(long) { return 3; }
struct Base { constexpr operator Fn() const { return first; } };
struct Derived : Base { constexpr operator Fn() const { return second; } };
struct Other { constexpr operator decltype(&third)() const { return third; } };
struct Both : Base, Other {};
static_assert(Derived{}(0) == 2);
static_assert(Both{}(0) == 1);
static_assert(Both{}(0L) == 3);
