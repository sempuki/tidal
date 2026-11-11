// tidal-rules: LAM-5 LAM-18
// tidal-expect: run
// tidal-note: Roadmap round 14, programmer reduction generic_lambda_reference_capture_member_pointer.cpp. The default capture [&] captures the pointer to member function pick by reference (LAM-5). The body of the generic lambda applies pick to its parameter with .* and calls the result (LAM-18). An assignment to pick after the lambda is made changes the function that the lambda calls.
struct Meter {
  int count;
  constexpr int doubled() const { return 2 * count; }
  constexpr int tripled() const { return 3 * count; }
};
constexpr int pick_and_call() {
  int (Meter::*pick)() const = &Meter::doubled;
  auto call = [&](auto&& m) { return (m.*pick)(); };
  int first = call(Meter{4});
  pick = &Meter::tripled;
  const Meter held{5};
  return first * 100 + call(held);
}
static_assert(pick_and_call() == 815);
int main() { return pick_and_call() == 815 ? 0 : 1; }
