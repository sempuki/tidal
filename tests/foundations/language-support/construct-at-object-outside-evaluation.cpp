// tidal-rules: LSUP-10 CE-5
// tidal-expect: reject
// tidal-error-clang: not an integral constant expression
// tidal-error-gcc: non-constant condition for static assertion
// tidal-note: The pointer points to a variable whose lifetime did not begin in the evaluation, so the placement new in std::construct_at does not run during constant evaluation.
inline constexpr void* operator new(decltype(sizeof 0), void* at) noexcept { return at; }
namespace std {
template <class T, class... Args>
constexpr T* construct_at(T* location, Args&&... args) {
  return ::new (static_cast<void*>(location)) T(static_cast<Args&&>(args)...);
}
}
int global = 1;
constexpr bool reset() { std::construct_at(&global, 5); return true; }
static_assert(reset());
