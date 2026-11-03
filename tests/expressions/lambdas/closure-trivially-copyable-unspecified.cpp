// tidal-rules: LAM-20
// tidal-expect: accept
// tidal-note: LAM-20 leaves open whether a closure type is trivially copyable. Every compiler in the matrix gives a closure that captures an int by copy trivial copy and move constructors and a trivial destructor, so a union can hold one and copy it. Another implementation can delete the union's functions.
template <class T> union Wrap { T x; };
template <class T> constexpr bool copies = requires (const T& t) { new T(t); };
template <class T> constexpr bool moves = requires (T& t) { new T(static_cast<T&&>(t)); };
template <class T> constexpr bool destroys = requires (T* p) { p->~T(); };
inline int counter = 0;
inline auto by_copy = [count = 1] { return count; };
inline auto by_reference = [&counter = counter] { return counter; };
using Copy = Wrap<decltype(by_copy)>;
using Reference = Wrap<decltype(by_reference)>;
static_assert(copies<Copy> && moves<Copy> && destroys<Copy>);
static_assert(copies<Reference> && moves<Reference> && destroys<Reference>);
