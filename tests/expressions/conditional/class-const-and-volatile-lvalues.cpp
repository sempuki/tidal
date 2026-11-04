// tidal-rules: COND-16 COND-3 COND-5
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: incompatible operand types
// tidal-error-gcc: operands to .\?:. have different types
// tidal-note: Q-0378. Two lvalues of class type Box, one const and one volatile. Neither type has all the cv-qualifiers of the other, so neither operand converts. Clang and GCC reject. EDG accepts.
struct Box { };
const Box fixed{};
volatile Box shared{};
extern bool c;
void f() { (void)(c ? fixed : shared); }
