// tidal-rules: COND-11 COND-5
// tidal-expect: reject
// tidal-error-clang: incompatible operand types
// tidal-error-gcc: operands to .\?:. have different types
// tidal-error-edg: operand types are incompatible
// tidal-note: No built-in candidate takes an int and an int pointer.
struct ToInt { operator int(); };
struct ToPointer { operator int*(); };
extern bool c;
ToInt i;
ToPointer p;
auto x = c ? i : p;
