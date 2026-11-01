// tidal-rules: DEP-13
// tidal-expect: reject
// tidal-error-clang: found in multiple base classes
// tidal-error-gcc: request for member .m. is ambiguous
// tidal-error-edg: is ambiguous
struct Fixed { int m = 1; };
struct Extra { int m = 2; };
template <class T> struct Both : Fixed, T {
  int get() { return this->m; }
};
int x = Both<Extra>{}.get();
