// tidal-rules: CNS-13
// tidal-expect: reject
// tidal-expect-clang-18: accept
// tidal-error-clang: no viable candidate for explicit instantiation
// tidal-error-gcc: does not match any template declaration
// tidal-error-edg: declaration is incompatible
// tidal-note: The constraint of f fails for char. Clang 18 accepts the explicit instantiation (Q-0291).
template <class T> concept Wide = sizeof(T) > 1;
template <class T> struct Box { void f() requires Wide<T> {} };
template void Box<char>::f();
