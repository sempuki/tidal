// tidal-rules: OVL-9 OVL-35
// tidal-expect: reject
// tidal-error-clang: cannot increment value of type .Count.
// tidal-error-gcc: no match for .operator\+\+.
// tidal-error-edg: no operator .\+\+. matches these operands
// tidal-note: The increment candidates take a reference to an arithmetic type. The conversion function returns an int prvalue, which no int& parameter takes, so no candidate is viable.
struct Count { operator int() const; };
void step(Count c) { ++c; }
