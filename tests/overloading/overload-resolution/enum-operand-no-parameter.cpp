// tidal-rules: OPR-15 OVL-8
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression
// tidal-error-gcc: no match for .operator\+.
// tidal-error-edg: no operator .\+. matches these operands
// tidal-note: No parameter of operator+ is the enumeration, so the operator is not a candidate although Wrap converts from Mode. No candidate remains.
enum class Mode { on };
struct Wrap { Wrap(Mode); };
bool operator+(Wrap, Wrap);
auto r = Mode::on + Mode::on;
