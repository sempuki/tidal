// tidal-rules: LSUP-6 NEW-31
// tidal-expect: run
namespace std {
using size_t = decltype(sizeof 0);
struct nothrow_t { explicit nothrow_t() = default; };
extern const nothrow_t nothrow;
}
void* operator new(std::size_t, const std::nothrow_t&) noexcept;
void* operator new[](std::size_t, const std::nothrow_t&) noexcept;
void operator delete(void*, const std::nothrow_t&) noexcept;
void operator delete[](void*, const std::nothrow_t&) noexcept;
void* volatile kept = nullptr;
static_assert(noexcept(new (std::nothrow) int));
int main() {
  int* one = new (std::nothrow) int(4);
  if (one == nullptr || *one != 4) return 1;
  delete one;
  int* many = new (std::nothrow) int[3]{1, 2, 3};
  if (many == nullptr || many[2] != 3) return 2;
  delete[] many;
  kept = operator new(decltype(sizeof 0)(-1) / 2, std::nothrow);
  if (kept != nullptr) return 3;
  return 0;
}
