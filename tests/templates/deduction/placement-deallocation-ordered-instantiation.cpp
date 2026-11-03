// tidal-rules: DED-39 ORD-9
// tidal-expect: accept
// tidal-note: Both deallocation templates match the placement form with Arena*. The body of the template with T fails if the template is instantiated. EDG runs no programs, and its acceptance shows that EDG chooses the template with T* (Q-0219). GCC chooses the same template. Clang chooses neither and also accepts.
using size = decltype(sizeof 0);
template <class T> constexpr bool never = false;
struct Arena {};
struct Widget {
  static void* operator new(size n, Arena*) { return ::operator new(n); }
  template <class T> static void operator delete(void* p, T*) { ::operator delete(p); }
  template <class T> static void operator delete(void* p, T) { static_assert(never<T>); ::operator delete(p); }
  static void operator delete(void* p) { ::operator delete(p); }
  Widget() { throw 0; }
};
int main() {
  Arena a;
  try { new (&a) Widget; } catch (int) {}
  return 0;
}
