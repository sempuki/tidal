// tidal-rules: LAM-5 LAM-18
// tidal-expect: run
// tidal-note: Roadmap round 14, programmer probes generic_lambda_reference_capture_closure_in_template.cpp and generic_lambda_reference_capture_closure_constant.cpp. In a function template, a generic lambda with the default capture [&] captures the closure parameter f by reference (LAM-5) and calls it (LAM-18). The call works in constant evaluation and at run time, and the closure of the run-time call changes a local of main through its own capture.
template <class F> constexpr int twice_through(F f, int start) {
  auto step = [&](auto v) { return f(v); };
  return step(step(start));
}
static_assert(twice_through([](int v) { return v + 3; }, 1) == 7);
int main() {
  int calls = 0;
  int r = twice_through([&calls](int v) { ++calls; return v * 2; }, 5);
  return r == 20 && calls == 2 ? 0 : 1;
}
