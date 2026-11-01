// tidal-rules: NEW-23 DED-39
// tidal-expect: accept
// tidal-note: The body of the template fails if the template is instantiated. EDG runs no programs, and its acceptance shows that EDG chooses the function over the template (Q-0220). Clang chooses neither and also accepts.
using size = decltype(sizeof 0);
template <class T> constexpr bool never = false;
struct Arena {};
struct Gadget {
  static void* operator new(size n, Arena*) { return ::operator new(n); }
  static void operator delete(void* p, Arena*) { ::operator delete(p); }
  template <class T> static void operator delete(void* p, T*) { static_assert(never<T>); ::operator delete(p); }
  static void operator delete(void* p) { ::operator delete(p); }
  Gadget() { throw 0; }
};
int main() {
  Arena a;
  try { new (&a) Gadget; } catch (int) {}
  return 0;
}
