// tidal-rules: NEW-25 NEW-13
// tidal-expect: reject
// tidal-error-clang: private member
// tidal-error-gcc: is private within this context
// tidal-error-edg: is inaccessible
// tidal-note: G155 (front-end report 23). The class's own operator new is private, and main is not a member or a friend, so the new expression selects an allocation function that it cannot access (NEW-13, NEW-25).
using size = decltype(sizeof 0);
struct Pinned {
  static void operator delete(void* p) { ::operator delete(p); }
private:
  static void* operator new(size n) { return ::operator new(n); }
};
int main() {
  Pinned* p = new Pinned;
  delete p;
  return 0;
}
