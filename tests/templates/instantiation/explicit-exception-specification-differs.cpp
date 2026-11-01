// tidal-rules: INST-22
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: exception specification in explicit instantiation does not match
// tidal-error-edg: exception specification is incompatible
// tidal-note: The explicit instantiation says noexcept, and the template does not. Clang and EDG reject the explicit instantiation, GCC accepts it (Q-0255).
template <class T> void run(T) {}
template void run(int) noexcept;
