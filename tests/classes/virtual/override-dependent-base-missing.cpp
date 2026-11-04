// tidal-rules: VIRT-3 DEP-28
// tidal-expect: reject
// tidal-error-clang: only virtual member functions can be marked 'override'
// tidal-error-gcc: marked .override., but does not override
// tidal-error-edg: declared with "override" does not
// tidal-note: The base is a template parameter. The check of override waits for each specialization. Wrapper<HasRun> is valid, and Wrapper<NoRun> is not, because NoRun has no run to override.
struct HasRun { virtual int run() { return 1; } };
struct NoRun { virtual int stop() { return 1; } };
template <class Base> struct Wrapper : Base { int run() override { return 2; } };
Wrapper<HasRun> good;
Wrapper<NoRun> bad;
