// tidal-rules: REF-6
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: cannot be used to initialize
// tidal-note: Q-0165. The referenced type is a function type, so REF-6 binds the reference as = does, and void() can refer to a noexcept function. EDG rejects the function in braces.
void quiet() noexcept {}
void (&call)(){quiet};
