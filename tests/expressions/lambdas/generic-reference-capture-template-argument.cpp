// tidal-rules: LAM-5 LAM-18 DED-1
// tidal-expect: run
// tidal-note: Roadmap round 14, programmer reduction generic_lambda_reference_capture_template_argument.cpp (from the tries of fields.cpp). The default capture [&] captures the class object tally by reference (LAM-5). The body passes the parameter and tally to the function template add (LAM-18). Deduction takes T from the parameter for each call (DED-1), and the reference parameter of add binds to tally, so the three calls change one object.
struct Tally { long total = 0; };
template <class T> constexpr void add(T value, Tally& into) { into.total += value; }
constexpr long record_three() {
  Tally tally;
  auto record = [&](auto value) { add(value, tally); };
  record(3);
  record(4L);
  record(static_cast<short>(2));
  return tally.total;
}
static_assert(record_three() == 9);
int main() { return record_three() == 9 ? 0 : 1; }
