// tidal-rules: COND-3 COND-5 COND-16
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: incompatible operand types
// tidal-error-gcc: operands to .\?:. have different types
// tidal-note: Decision 0059, item F2 (I4-8). Q-0378. Two xvalues of class type Box, one const and one volatile. Neither type has all the cv-qualifiers of the other, so neither operand converts. Clang and GCC reject. EDG accepts, as for the lvalue form.
template <class T> T&& move_of(T& t) { return static_cast<T&&>(t); }
struct Box { };
const Box fixed{};
volatile Box shared{};
extern bool c;
void f() { (void)(c ? move_of(fixed) : move_of(shared)); }
