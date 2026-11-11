// tidal-rules: DEP-15 DEP-1
// tidal-expect: run
// tidal-note: Roadmap round 10, programmer reduction member_of_other_specialization_by_constant_argument.cpp. Inside Gauge<N>, the argument list of Gauge<0> differs from the template's own parameters, so Gauge<0> names another specialization (DEP-15). The list has no dependent argument, so Gauge<0> is not dependent (DEP-1), and the definition looks up the member level. Gauge<0> has the member, and every compiler runs the program.
template <int N> struct Gauge {
  int level = N;
  int diff(const Gauge<0>& o) const { return level - o.level; }
};
int main() {
  Gauge<0> zero;
  Gauge<3> three;
  return three.diff(zero) == 3 ? 0 : 1;
}
