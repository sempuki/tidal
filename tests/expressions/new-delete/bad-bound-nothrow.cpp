// tidal-rules: NEW-36 LSUP-6
// tidal-expect: run
// tidal-expect-gcc: run-fail
// tidal-note: Q-0302. GCC throws for a bad bound also when the allocation function is non-throwing.
namespace std {
using size_t = decltype(sizeof 0);
struct nothrow_t { explicit nothrow_t() = default; };
extern const nothrow_t nothrow;
}
int calls = 0;
alignas(16) unsigned char pool[256];
void* operator new[](std::size_t n, const std::nothrow_t&) noexcept {
  ++calls;
  return n > sizeof pool ? nullptr : pool;
}
long bound(long n) { return n; }
std::size_t huge(std::size_t n) { return n; }
template <class F> int check(F make) {
  try { return make() == nullptr ? 0 : 1; } catch (...) { return 2; }
}
int main() {
  if (check([] { return new (std::nothrow) int[bound(-1)]; }) != 0) return 1;
  if (check([] { return new (std::nothrow) int[bound(1)]{1, 2, 3}; }) != 0) return 2;
  if (check([] { return new (std::nothrow) int[huge(std::size_t(-1) / 2)]; }) != 0) return 3;
  return 0;
}
