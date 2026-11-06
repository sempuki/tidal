// notes 14: const T& with a function argument. T is the function type.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
template <class T> Tag<T> f(const T&);
int g(char) { return 0; }
int main() { return same<decltype(f(g)), Tag<int(char)>> ? 0 : 1; }
