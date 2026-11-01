// tidal-rules: DEP-26
// tidal-expect: reject
// tidal-error-clang: does not refer to a template
// tidal-error-gcc: no matching function for call to .Has::plain<int>\(\).
// tidal-error-edg: must be a template
struct Has { static int plain() { return 0; } };
template <class T> int run() { return T::template plain<int>(); }
int x = run<Has>();
