// tidal-rules: NAM-28 NAM-8
// tidal-expect: reject
// tidal-error-clang: undeclared identifier .probe.
// tidal-error-gcc: probe. was not declared in this scope
// tidal-error-edg: identifier .probe. is undefined
namespace n {
struct maker {
  template <class T> friend int probe(T) { return 1; }
  static auto make() { struct local { }; return local{}; }
};
}
int read() { return probe(n::maker::make()); }
