// tidal-rules: OVL-13 OVL-35
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression
// tidal-error-gcc: no match for .operator==.
// tidal-error-edg: no operator .==. matches these operands
// tidal-note: The swapped candidate from the member makes the right operand its object. No conversion reaches the object, so Maker does not become Right.
struct Left {};
struct Right { bool operator==(const Left&) const; };
struct Maker { operator Right() const; };
bool r = Left{} == Maker{};
