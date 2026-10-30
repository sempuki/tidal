// tidal-rules: CPY-13
// tidal-expect: reject
// tidal-flags: -Wall -Werror
namespace std {
template <class T> struct remove_reference { using type = T; };
template <class T> struct remove_reference<T&> { using type = T; };
template <class T> constexpr typename remove_reference<T>::type&& move(T&& t) noexcept {
  return static_cast<typename remove_reference<T>::type&&>(t);
}
}
struct S { S() {} S(const S&) {} S(S&&) {} };
S make() { S s; return std::move(s); }
