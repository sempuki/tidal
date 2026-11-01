// The library function that constant evaluation lets run a placement
// new expression, declared as a program that includes no header declares
// it (LSUP-10).
inline constexpr void* operator new(decltype(sizeof 0), void* place) noexcept { return place; }
inline void operator delete(void*, void*) noexcept {}
namespace std {
template <class T, class... Args>
constexpr T* construct_at(T* location, Args&&... args) {
  return ::new (static_cast<void*>(location)) T(static_cast<Args&&>(args)...);
}
}
