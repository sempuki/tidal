// tidal-rules: NAM-27
// tidal-expect: accept
// tidal-note: A function template in the set adds what its parameter types and return type add, if they do not depend on its template parameters. A plain function in a set with a template adds what its own type adds.
namespace N {
struct A {};
int by_parameter(void (*)(int, A));
int by_return(A (*)(int));
int by_plain(void (*)(A));
}
template <class T> void p(T, N::A);
template <class T> N::A r(T);
void q(N::A);
template <class T> void q(T, int);
int a = by_parameter(p);
int b = by_return(r);
int c = by_plain(q);
