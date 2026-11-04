// tidal-rules: MPTR-4
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: static assertion failed
// tidal-note: Q-0353. A pointer to member of Base, converted to a pointer to member of Derived, names the same member after it passes through a function result or a constexpr array. During constant evaluation EDG 7.0 loses the member in both cases.
struct Base {
  int value = 0;
  constexpr void bump() { value += 1; }
};
struct Derived : Base {
  int extra = 0;
};

using DerivedData = int Derived::*;
using DerivedCall = void (Derived::*)();

constexpr DerivedData pick_data() { return &Base::value; }
constexpr void (Base::*calls[1])() = {&Base::bump};

constexpr int run() {
  Derived d;
  d.*pick_data() = 7;
  DerivedCall call = calls[0];
  (d.*call)();
  return d.value;
}
static_assert(run() == 8);
