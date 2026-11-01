// tidal-rules: NEW-21 NEW-23
// tidal-expect: run
// tidal-note: The array placement form operator delete[] frees the storage, and the single-object placement form is not called.
using size = decltype(sizeof 0);
int freed = 0;
struct Tag {};
struct Gadget {
  static void* operator new[](size n, Tag) { return ::operator new(n); }
  static void operator delete[](void* p, Tag) { freed = 1; ::operator delete(p); }
  static void operator delete(void* p, Tag) { freed = 2; ::operator delete(p); }
  static void operator delete[](void* p) { freed = 3; ::operator delete(p); }
  Gadget() { throw 0; }
};
int main() {
  try { new (Tag{}) Gadget[2]; } catch (int) {}
  return freed == 1 ? 0 : 10 + freed;
}
