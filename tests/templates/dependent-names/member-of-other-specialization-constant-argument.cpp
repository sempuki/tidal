// tidal-rules: DEP-15 DEP-39
// tidal-expect: run
// tidal-note: Roadmap round 10, programmer reduction member_of_other_specialization_by_constant_argument.cpp. Inside Gauge<N>, the argument list of Gauge<0> differs from the template's own parameters, so Gauge<0> names another specialization, which is dependent (DEP-15). The member access o.level is looked up at the instantiation (DEP-39), where Gauge<0> has the member.
template <int N> struct Gauge {
  int level = N;
  int diff(const Gauge<0>& o) const { return level - o.level; }
};
int main() {
  Gauge<0> zero;
  Gauge<3> three;
  return three.diff(zero) == 3 ? 0 : 1;
}
