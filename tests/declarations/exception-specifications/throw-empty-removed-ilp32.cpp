// tidal-rules: EXS-5
// tidal-expect: accept
// tidal-flags: -m32
// tidal-expect-edg: reject
// tidal-error-edg: throw\(\).*not part of
// tidal-note: Q-0077. -m32. Clang and GCC accept throw() as noexcept. EDG rejects it.
void f() throw();
static_assert(noexcept(f()));
