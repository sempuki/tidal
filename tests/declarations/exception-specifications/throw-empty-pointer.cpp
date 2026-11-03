// tidal-rules: EXS-5
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: throw\(\).*not part of
// tidal-note: Q-0077. throw() on a pointer to function. Clang and GCC accept it as noexcept. EDG rejects it.
void g() noexcept;
void (*p)() throw() = g;
static_assert(noexcept(p()));
