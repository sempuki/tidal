// tidal-rules: NAM-27
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: use of undeclared identifier .take.
// tidal-note: Q-0311. The argument is the template-id h<M::B>. GCC adds the namespace of the type argument M::B and finds M::take. Clang adds nothing for the template argument.
namespace M {
struct B {};
int take(void (*)(B));
}
template <class T> void h(T);
int r = take(h<M::B>);
