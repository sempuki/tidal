// tidal-rules: NEW-37 LSUP-6
// tidal-expect: accept
// tidal-expect-gcc-14: reject
// tidal-expect-edg: reject
// tidal-note: Q-0303. GCC 14 and EDG do not evaluate the nothrow forms during constant evaluation.
// tidal-error-gcc-14: call to non-.constexpr. function .void\* operator new\(std::size_t, const std::nothrow_t&\)
// tidal-error-edg: placement new-expression cannot be constant-evaluated
namespace std {
using size_t = decltype(sizeof 0);
struct nothrow_t { explicit nothrow_t() = default; };
inline constexpr nothrow_t nothrow{};
}
void* operator new(std::size_t, const std::nothrow_t&) noexcept;
void* operator new[](std::size_t, const std::nothrow_t&) noexcept;
constexpr int make() {
  int* one = new (std::nothrow) int(4);
  int* many = new (std::nothrow) int[2]{5, 6};
  int v = *one + many[1];
  delete one;
  delete[] many;
  return v;
}
static_assert(make() == 10);
