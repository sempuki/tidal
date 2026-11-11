// The two-step swap: a using-declaration of a generic swap, then an unqualified call
// that finds the class's own swap by argument-dependent lookup.
namespace util {
template <class T> void swap(T& a, T& b) { T t = static_cast<T&&>(a); a = static_cast<T&&>(b); b = static_cast<T&&>(t); }
}
namespace app {
struct Buf { int* p; int calls = 0; };
inline void swap(Buf& a, Buf& b) { int* t = a.p; a.p = b.p; b.p = t; ++a.calls; }
}
template <class T> void exchange_both(T& a, T& b) { using util::swap; swap(a, b); }
int main() {
  int x = 1, y = 2;
  app::Buf a{&x}, b{&y};
  exchange_both(a, b);
  int i = 5, j = 6;
  exchange_both(i, j);
  return *a.p == 2 && a.calls == 1 && i == 6 ? 0 : 1;
}
