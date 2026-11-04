// tidal-rules: REF-11
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-expect-gcc: accept
// tidal-error-edg: cannot be used to initialize
// tidal-note: Q-0165. The type of quiet has noexcept, so the braced element is not of the referenced type, and a function type has no temporary. EDG rejects the function in braces. Clang and GCC bind the reference as after =.
void quiet() noexcept {}
void (&call)(){quiet};
