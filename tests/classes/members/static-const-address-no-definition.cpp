// tidal-rules: CLS-21
// tidal-expect: link-fail
// tidal-error-clang: undefined reference to .S::m
// tidal-error-gcc: undefined reference to .S::m
// tidal-note: The program takes the address of a const static member of enumeration type that has no definition outside the class.
enum Mode { slow, fast };
struct S { static const Mode m = fast; };
const Mode* p = &S::m;
int main() { return *p == fast ? 0 : 1; }
