// tidal-rules: TR-16
// tidal-expect: run
// tidal-note: The constant rounds the sum to float. At run time an implementation can keep the sum in a wider type (MUL-17), so the run can give a value that differs in the last bits. Both results are allowed.
constexpr float sum(float a, float b) { return a + b; }
constexpr float at_translation = sum(0.1f, 0.2f);
volatile float first = 0.1f;
volatile float second = 0.2f;
int main() {
  float at_run = sum(first, second);
  float gap = at_run - at_translation;
  bool known = at_translation > 0.29999f && at_translation < 0.30001f;
  bool close = gap > -0.000001f && gap < 0.000001f;
  return known && close ? 0 : 1;
}
