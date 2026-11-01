// tidal-rules: DED-32
// tidal-expect: accept
// tidal-note: The friend declaration peek<>(Box*) matches both templates. peek(T*) is more specialized, so that specialization is the friend and can read the private member.
template <class T> constexpr int peek(T) { return 0; }
template <class T> constexpr int peek(T* p) { return p->secret; }
class Box {
  int secret = 5;
  friend constexpr int peek<>(Box*);
};
constexpr int read() { Box b; return peek(&b); }
static_assert(read() == 5);
