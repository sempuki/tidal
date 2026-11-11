// tidal-rules: VC-4 OVL-14
// tidal-expect: run
// tidal-note: P3-11. Action has deleted copy operations and a converting constructor template. Action<int> a = closure; calls the converting constructor, and its result initializes a directly, so no copy constructor is needed. With an explicit converting constructor, the same form is rejected (OVL-14).
template <class A> struct Action {
  int seen;
  template <class F> Action(F f) : seen(f(A(2))) {}
  Action(const Action&) = delete;
  Action& operator=(const Action&) = delete;
};
int main() {
  int base = 3;
  Action<int> a = [&base](int v) { return base + v; };
  return a.seen - 5;
}
