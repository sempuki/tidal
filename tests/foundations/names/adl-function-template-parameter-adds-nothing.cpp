// tidal-rules: NAM-8 NAM-27
// tidal-expect: reject
// tidal-error-clang: use of undeclared identifier .take.
// tidal-error-gcc: .take. was not declared in this scope
// tidal-error-edg: identifier .take. is undefined
// tidal-note: The only parameter type of h is its template parameter, so the set adds no namespace, and lookup does not find M::take.
namespace M {
struct B {};
int take(void (*)(B));
}
template <class T> void h(T);
int r = take(h);
