// tidal-rules: NEW-41
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=address,undefined -fno-sanitize-recover=all
// tidal-note: The non-throwing forms and the aligned array form of the global deallocation functions, called by name with a null pointer.
namespace std {
using size_t = decltype(sizeof 0);
enum class align_val_t : size_t {};
struct nothrow_t { explicit nothrow_t() = default; };
extern const nothrow_t nothrow;
}
void operator delete(void*, const std::nothrow_t&) noexcept;
void operator delete[](void*, const std::nothrow_t&) noexcept;
void operator delete(void*, std::align_val_t, const std::nothrow_t&) noexcept;
int main() {
  void* none = nullptr;
  ::operator delete(none, std::nothrow);
  ::operator delete[](none, std::nothrow);
  ::operator delete[](none, std::align_val_t(64));
  ::operator delete(none, std::align_val_t(64), std::nothrow);
  return 0;
}
