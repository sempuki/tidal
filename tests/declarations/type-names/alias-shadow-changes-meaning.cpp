// tidal-rules: TYN-5
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-note: Q-0066. GCC rejects a class member declaration that changes the meaning of a name already used in the class. Clang and EDG accept it.
typedef int I;
struct S { I a; typedef long I; };
