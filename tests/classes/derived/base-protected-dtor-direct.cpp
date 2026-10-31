// tidal-rules: ACC-4
// tidal-expect: reject
// tidal-error-clang: has protected destructor
// tidal-error-gcc: protected within this context
// tidal-error-edg: is inaccessible
class B { protected: ~B() {} };
void f() { B b; }
