// tidal-rules: OVL-11
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: invalid operands to binary expression
// tidal-error-gcc: no match for .operator\+.
// tidal-note: Q-0141. The class operand converts to double and stops there. EDG also converts the double to the candidate's integer parameter.
struct ToDouble { operator double() const; };
int* p = nullptr;
auto r = p + ToDouble{};
