// tidal-rules: LAM-5 LAM-18
// tidal-expect: run
// tidal-note: Roadmap round 14, programmer reduction generic_lambda_reference_capture_object_call.cpp. The default capture [&] captures the class object scale by reference (LAM-5). The body of the generic lambda calls the captured object, and each argument type selects a call operator of the object (LAM-18). A change to scale after the lambda is made shows in each call, at run time and in constant evaluation.
struct Scale {
  int factor;
  constexpr int operator()(int v) const { return factor * v; }
  constexpr long operator()(long v) const { return factor * v + 1; }
};
constexpr int apply_both() {
  Scale scale{2};
  auto times = [&](auto v) { return scale(v); };
  scale.factor = 3;
  return times(4) + static_cast<int>(times(5L));
}
static_assert(apply_both() == 28);
int main() {
  Scale scale{5};
  auto times = [&](auto v) { return scale(v); };
  scale.factor = 7;
  return times(2) == 14 && times(1L) == 8 ? 0 : 1;
}
