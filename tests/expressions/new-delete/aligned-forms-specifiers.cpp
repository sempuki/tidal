// tidal-rules: NEW-26
// tidal-expect: accept
namespace std {
using size_t = decltype(sizeof 0);
enum class align_val_t : size_t {};
}
static_assert(!noexcept(operator new(1, std::align_val_t(32))));
static_assert(!noexcept(operator new[](1, std::align_val_t(32))));
static_assert(noexcept(operator delete(nullptr, std::align_val_t(32))));
static_assert(noexcept(operator delete[](nullptr, std::align_val_t(32))));
