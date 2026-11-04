// tidal-rules: TR-16
// tidal-expect: run
// tidal-note: The constant rounds each step to double. At run time an implementation can keep extra precision (MUL-17), so the run can give the first value. Both results are allowed.
constexpr double scaled(double v) { return v * 3.0 / 3.0; }
constexpr double at_translation = scaled(0.1);
volatile double tenth = 0.1;
int main() {
  double at_run = scaled(tenth);
  bool known = at_translation == 0.1 || at_translation == 0.10000000000000002;
  bool either = at_run == at_translation || at_run == 0.1 || at_run == 0.10000000000000002;
  return known && either ? 0 : 1;
}
