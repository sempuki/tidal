// tidal-rules: DEP-30 DEP-15
// tidal-expect: reject
// tidal-error-clang: no member named .missing. in .Meter<0>.
// tidal-error-gcc: has no member named .missing.
// tidal-error-edg: has no member
// tidal-note: Roadmap round 10 programmer report, section 6, with the writer probe reports/clean-room/roadmap-10/writer-probes/f3b-dep-15-constant-argument.json. Inside Meter<N>, Meter<0> is not dependent (DEP-15), so the member access o.missing is an independent construct that is not valid. DEP-30 lets an implementation leave it undiagnosed while no specialization uses it. Every compiler rejects the definition, though the program never calls compare.
template <int N> struct Meter {
  int level = N;
  int compare(const Meter<0>& o) const { return level - o.missing; }
};
int main() {
  Meter<3> m;
  return m.level == 3 ? 0 : 1;
}
