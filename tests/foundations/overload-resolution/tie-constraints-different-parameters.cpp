// tidal-rules: OVL-36 OVL-28
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: call to .f. is ambiguous
// tidal-error-gcc: call of overloaded .f\(int&\). is ambiguous
// tidal-note: Q-0146. Neither template is more specialized, because the parameters T and const T& differ. The constraint does not decide. EDG chooses the constrained template.
template <class T> void f(const T&);
template <class T> requires (sizeof(T) > 1) void f(T);
int n = 0;
void use() { f(n); }
