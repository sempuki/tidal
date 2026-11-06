// notes 1: a prvalue of type const int gives T = int.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
const int give() { return 1; }
template <class T> Tag<T> fwd(T&&);
int main() { return same<decltype(fwd(give())), Tag<int>> ? 0 : 1; }
