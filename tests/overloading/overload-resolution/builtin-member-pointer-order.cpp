// tidal-rules: OPR-15 OVL-9
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression
// tidal-error-gcc: no match for .operator<.
// tidal-error-edg: no operator .<. matches these operands
// tidal-note: Member pointers have no relational candidates.
struct Item { int v; };
struct ToMember { operator int Item::*() const; };
bool r = ToMember{} < ToMember{};
