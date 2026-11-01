// tidal-rules: DEP-36
// tidal-expect: run
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct Base { using U = int; };
template <class U> struct Inside : Base { bool get() { return same<U, int>; } };
template <class T> struct Outside : Base { bool get(); };
template <class U> bool Outside<U>::get() { return same<U, int>; }
int main() { return Inside<double>{}.get() && Outside<double>{}.get() ? 0 : 1; }
