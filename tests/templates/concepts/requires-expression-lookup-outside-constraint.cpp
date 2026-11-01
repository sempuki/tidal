// tidal-rules: CPT-26 DEP-9
// tidal-expect: run
// tidal-note: The requires expressions in the if constexpr condition and in the variable template are no constraints. Their dependent call use(t) finds functions by ordinary lookup at the definition and by argument-dependent lookup at the instantiation.
template <class T> int probe(T t) {
  if constexpr (requires { use(t); }) return 1;
  else return 2;
}
template <class T> constexpr bool usable = requires(T t) { use(t); };
namespace lib { struct A {}; int use(A); }
namespace other { struct B {}; }
int use(other::B);
int main() {
  if (probe(lib::A{}) != 1) return 1;
  if (probe(other::B{}) != 2) return 2;
  if (!usable<lib::A>) return 3;
  return usable<other::B> ? 4 : 0;
}
