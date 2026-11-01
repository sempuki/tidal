// tidal-rules: DCMP-13 DCMP-12
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression
// tidal-error-gcc: no match for .operator==.
// tidal-error-edg: no operator .==. matches
// tidal-note: The using-declaration puts the != of other into the namespace of the ==. The != matches, so DCMP-6 removes the reversed ==.
namespace units { struct Meter {}; bool operator==(const Meter&, int); }
namespace other { bool operator!=(const units::Meter&, int); }
namespace units { using other::operator!=; }
bool check(units::Meter m) { return 1 == m; }
