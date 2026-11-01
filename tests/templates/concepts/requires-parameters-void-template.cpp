// tidal-rules: CPT-28
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: static assertion failed
// tidal-note: (void) means no parameters. Clang finds the requires expression false inside a template (Q-0290).
template <class T> concept Makes = requires (void) { T(); };
static_assert(Makes<int>);
