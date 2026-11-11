// construct_at on a member of a union member that is not active. GCC and
// Clang reject the constant; EDG accepts; the front end rejects with UNI-6.
// UNI-14 covers only an object of the member's own type.
namespace std {
template <class T, class... A>
constexpr T* construct_at(T* p, A&&... a) { return ::new (static_cast<void*>(p)) T(static_cast<A&&>(a)...); }
}
inline void* operator new(decltype(sizeof 0), void* p) noexcept { return p; }
union Inner { int i; double d; constexpr Inner() {} };
union Outer { char c; Inner in; constexpr Outer() {} };
constexpr int f() {
  Outer o;
  std::construct_at(&o.in.d, 2.5);
  return o.in.d == 2.5 ? 0 : 1;
}
static_assert(f() == 0);
int main() { return f(); }
