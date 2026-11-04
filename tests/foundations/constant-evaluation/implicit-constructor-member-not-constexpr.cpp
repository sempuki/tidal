// tidal-rules: CE-2
// tidal-expect: reject
// tidal-error-clang: non-constexpr constructor .Part.|non-literal type .Holder.
// tidal-error-gcc: call to non-.constexpr. function .Holder::Holder\(\).
// tidal-error-edg: cannot call non-constexpr function .Holder::Holder\(\).
// tidal-note: The implicit constructor of Holder calls the constructor of Part, which is not constexpr.
struct Part {
  int v = 1;
  Part() {}
};
struct Holder { Part part; };
constexpr int read() {
  Holder h;
  return h.part.v;
}
static_assert(read() == 1);
