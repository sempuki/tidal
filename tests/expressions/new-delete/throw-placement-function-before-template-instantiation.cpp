// tidal-rules: NEW-24 DED-39
// tidal-expect: accept
// tidal-note: A function and a function template both match the placement form. The body of the template fails if the template is instantiated. No compiler instantiates the template. Clang calls neither function, and GCC calls the function (Q-0220).
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
