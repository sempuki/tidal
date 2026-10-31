// tidal-rules: NS-10
// tidal-expect: reject
// tidal-error-clang: undeclared identifier .x
// tidal-error-gcc: x. was not declared in this scope
// tidal-error-edg: identifier .x. is undefined
namespace N { int x; }
void f() { using namespace N; }
int y = x;
