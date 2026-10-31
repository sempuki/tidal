// tidal-rules: OVL-36 OVL-10 OPR-15
// tidal-expect: reject
// tidal-error-clang: use of overloaded operator .==. is ambiguous
// tidal-error-gcc: ambiguous overload for .operator==.
// tidal-error-edg: more than one operator .==. matches these operands
// tidal-note: The non-member takes references, so its parameter list differs from the built-in candidate's. Both stay, and their sequences tie.
enum Color { red, blue };
bool operator==(const Color&, const Color&);
bool r = red == blue;
