// tidal-rules: NEW-21 NEW-22
// tidal-expect: run
// tidal-note: The class declares both usual forms. The form with one parameter frees the storage and sets freed to 1.
using size = decltype(sizeof 0);
int freed = 0;
struct Gadget {
  static void* operator new(size n) { return ::operator new(n); }
  static void operator delete(void* p) { freed = 1; ::operator delete(p); }
  static void operator delete(void* p, size) { freed = 2; ::operator delete(p); }
  Gadget() { throw 0; }
};
int main() {
  try { new Gadget; } catch (int) {}
  return freed == 1 ? 0 : 10 + freed;
}
