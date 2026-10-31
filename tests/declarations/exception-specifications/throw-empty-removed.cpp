// tidal-rules: EXS-5
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-note: Q-0077. Clang and GCC accept throw() as noexcept. EDG rejects it.
void f() throw();
static_assert(noexcept(f()));
