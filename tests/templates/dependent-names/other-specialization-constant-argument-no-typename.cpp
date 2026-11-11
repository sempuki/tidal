// tidal-rules: DEP-15 DEP-1
// tidal-expect: run
// tidal-note: Roadmap round 10 programmer report, section 6, with the writer probe reports/clean-room/roadmap-10/writer-probes/f3b-dep-15-constant-argument.json. Inside Meter<N>, Meter<0> names another specialization (DEP-15) whose argument list has no dependent argument, so it is not a dependent type (DEP-1). Meter<0>::unit is a qualified name with a qualifier that is not dependent, and it names a type with no typename. Every compiler runs the program.
template <int N> struct Meter {
  using unit = int;
  int base() const { Meter<0>::unit zero = 0; return zero + N; }
};
int main() {
  Meter<3> m;
  return m.base() == 3 ? 0 : 1;
}
