// tidal-rules: VIRT-3 DEP-30
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: only virtual member functions can be marked 'override'
// tidal-note: The base does not depend on T, and no specialization is made. Any specialization would fail, because Base has no run. DEP-30 lets an implementation reject the template or accept it.
struct Base { virtual int stop(); };
template <class T> struct Wrapper : Base { int run() override; };
