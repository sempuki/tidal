// tidal-rules: DTOR-8
// tidal-expect: reject
// tidal-error-clang: does not match the type
// tidal-error-gcc: expected class-name before .decltype.
// tidal-error-edg: invalid destructor name
// tidal-note: Gap review RV6-A-11. decltype of an lvalue that is not a plain name, such as *p, gives S&, which does not denote the class type S. Clang and EDG reject the call for that reason. GCC rejects every decltype after ~ (Q-0089).
struct S { ~S() {} };
void finish(S* p) { p->~decltype(*p)(); }
