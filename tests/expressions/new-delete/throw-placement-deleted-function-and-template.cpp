// tidal-rules: NEW-24 DED-39
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: deleted function
// tidal-note: A deleted function and a function template both match the placement form. Two matches give no deallocation function, so the deleted function is not chosen. EDG chooses the deleted function and rejects the program. GCC accepts a deleted placement deallocation function in any case (Q-0221), so its result does not show its choice (Q-0220).
using size = decltype(sizeof 0);
struct Arena {};
struct Gadget {
  static void* operator new(size n, Arena*) { return ::operator new(n); }
  static void operator delete(void* p, Arena*) = delete;
  template <class T> static void operator delete(void* p, T*) { ::operator delete(p); }
  static void operator delete(void* p) { ::operator delete(p); }
  Gadget() { throw 0; }
};
int main() {
  Arena a;
  try { new (&a) Gadget; } catch (int) {}
  return 0;
}
