// tidal-rules: CNS-35 CPY-15
// tidal-expect: run
// tidal-expect-clang-18: reject
// tidal-error-clang-18: depends on itself
// tidal-note: W20-3, W20 reduction self-by-value-template-constraint.cpp. Q-0426. The check Holdable<Holder> considers Holder(h) with h a const Holder lvalue. The constructor template Holder(T) with T = Holder never gives a constructor (CPY-15), and the copy constructor serves the copy. Clang 22, GCC 14 and 16, and EDG accept the program. Clang 18 reports that the satisfaction of the constraint depends on itself.
struct Holder {
  Holder() = default;
  template <class T>
    requires requires(const T& t) { T(t); }
  Holder(T) {}
};
template <class T>
concept Holdable = requires(const T& t) { Holder(t); };
static_assert(Holdable<Holder>);
static_assert(Holdable<int>);
int main() {
  const Holder a;
  Holder b = a;
  (void)b;
  return 0;
}
