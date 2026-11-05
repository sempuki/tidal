// tidal-rules: CPY-3 CTOR-10
// tidal-expect: run
// tidal-note: The right operand is a prvalue of another specialization. The converting constructor template makes a temporary of the left operand's class, and the move assignment operator takes the temporary.
struct Shape { virtual ~Shape() {} virtual int sides() const { return 0; } };
struct Square : Shape { int sides() const override { return 4; } };
template <class T> struct Holder {
  T* ptr;
  explicit Holder(T* p) : ptr(p) {}
  Holder(Holder&& h) : ptr(h.ptr) { h.ptr = nullptr; }
  template <class U> Holder(Holder<U>&& h) : ptr(h.ptr) { h.ptr = nullptr; }
  Holder& operator=(Holder&& h) {
    if (this != &h) { delete ptr; ptr = h.ptr; h.ptr = nullptr; }
    return *this;
  }
  ~Holder() { delete ptr; }
};
int main() {
  Holder<Shape> s(new Shape);
  s = Holder<Square>(new Square);
  return s.ptr->sides() == 4 ? 0 : 1;
}
