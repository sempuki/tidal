// tidal-rules: ICS-10 OVL-35
// tidal-expect: reject
// tidal-error-clang: no viable overloaded .=.
// tidal-error-gcc: no match for .operator=.
// tidal-error-edg: no operator .=. matches these operands
// tidal-note: The built-in int& = int would need a user-defined sequence for its left operand, so it is not a candidate. The class's own assignment operators take only a Cell. The compound assignment += in builtin-increment-from-conversion.cpp has no such limit.
int value = 0;
struct Cell { operator int&() { return value; } };
void set(Cell c) { c = 1; }
