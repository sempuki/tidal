// std::construct_at in the shape that LSUP-10 gives, with the
// non-allocating placement forms of LSUP-7.
inline constexpr void* operator new(decltype(sizeof 0), void* place) noexcept { return place; }
inline void operator delete(void*, void*) noexcept {}
namespace std {
template <class T, class... Args>
constexpr T* construct_at(T* location, Args&&... args) {
  return ::new (static_cast<void*>(location)) T(static_cast<Args&&>(args)...);
}
}
