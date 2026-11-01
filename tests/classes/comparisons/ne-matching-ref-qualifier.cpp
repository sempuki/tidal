// tidal-rules: DCMP-13 DCMP-12
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: invalid operands to binary expression
// tidal-error-edg: no operator .==. matches
// tidal-note: Q-0190. The == has the ref-qualifier const& and the != has const&&. The ref-qualifier takes no part, so the != matches and DCMP-6 removes the reversed ==. GCC treats the two ref-qualifiers as different and accepts the comparison.
struct Meter {
  bool operator==(int) const&;
  bool operator!=(int) const&&;
};
bool check(Meter m) { return 1 == m; }
