// tidal-rules: BIT-8
// tidal-expect: reject
// tidal-error-clang: different enumeration types.*deprecated
// tidal-error-gcc: different enumeration types.*deprecated
// tidal-flags: -Werror -Wdeprecated-enum-enum-conversion
// tidal-note: The operator is ^, and the operands have two different enumeration types.
enum E { A = 1 };
enum F { B = 2 };
int f() { return A ^ B; }
