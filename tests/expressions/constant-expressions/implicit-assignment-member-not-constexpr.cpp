// tidal-rules: CE-2
// tidal-expect: reject
// tidal-error-clang: non-constexpr function .operator=.
// tidal-error-gcc: call to non-.constexpr. function
// tidal-error-edg: cannot call non-constexpr function "Holder::operator=
// tidal-note: The implicit assignment of Holder calls the assignment of Part, which is not constexpr.
struct Part {
  int v = 0;
  Part& operator=(const Part& o) { v = o.v; return *this; }
};
struct Holder { Part part; };
constexpr int assign() {
  Holder a, b;
  a = b;
  return a.part.v;
}
static_assert(assign() == 0);
