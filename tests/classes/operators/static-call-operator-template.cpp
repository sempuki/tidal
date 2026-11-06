// tidal-rules: OPR-3 CLS-15 DED-1
// tidal-expect: run
// tidal-note: Roadmap round 7, programmer reduction static_call_operator_template_run.cpp and static_call_operator_template_constant.cpp (reports/clean-room/roadmap-7/programmer.md, section 1.5).
struct Sum { template <class T> static constexpr T operator()(T a, T b) { return a + b; } };
static_assert(Sum{}(2, 3) == 5);
int main() { return Sum{}(2L, 3L) == 5 ? 0 : 1; }
