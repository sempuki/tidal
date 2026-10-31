// tidal-rules: OVL-36 OVL-33
// tidal-expect: reject
// tidal-error-clang: to be ambiguous
// tidal-error-gcc: these are ambiguous, even though the second is reversed
// tidal-error-edg: more than one operator .==. matches these operands
// tidal-note: The written a == b and the swapped b == a use one member that is not const. Each is better on one operand (ICS-30), so OVL-25 finds neither better and OVL-33 does not apply. DCMP-7 recommends a warning when strict mode is off.
struct Item {
  int v;
  bool operator==(const Item& o) { return v == o.v; }
};
bool same(Item a, Item b) { return a == b; }
