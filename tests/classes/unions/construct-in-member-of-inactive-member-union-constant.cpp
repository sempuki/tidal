// tidal-rules: UNI-14 CE-6
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: construction of subobject of member .inner. of union with no active member
// tidal-error-gcc: accessing uninitialized member
// tidal-note: Roadmap round 10, programmer reduction construct_in_member_of_inactive_union_member.cpp. Q-0425. The union Outer has no active member, and std::construct_at builds a double at the address of the member d of the member inner. The new double is not of the type of inner, so UNI-14 does not make inner active. GCC and Clang reject the constant, and EDG accepts it. The test follows GCC and Clang while the question is open.
inline constexpr void* operator new(decltype(sizeof 0), void* at) noexcept { return at; }
namespace std {
template <class T, class... A>
constexpr T* construct_at(T* p, A&&... a) {
  return ::new (static_cast<void*>(p)) T(static_cast<A&&>(a)...);
}
}
union Inner { int i; double d; constexpr Inner() {} };
union Outer { char tag; Inner inner; constexpr Outer() {} };
constexpr bool build() {
  Outer o;
  std::construct_at(&o.inner.d, 1.5);
  return o.inner.d == 1.5;
}
static_assert(build());
