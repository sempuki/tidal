// tidal-rules: ATTR-4
// tidal-expect: reject
// tidal-error-clang: Old. is deprecated
// tidal-error-gcc: Old. is deprecated
// tidal-flags: -Werror
// tidal-note: Gap review 2026-10-30.
enum E { Old [[deprecated]], New };
int f() { return Old; }
