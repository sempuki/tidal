// tidal-rules: CTOR-10 DED-1 OVL-14 JMP-3 CPY-11
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=address,undefined -fno-sanitize-recover=all
// tidal-note: Owner<Shape> is initialized from an Owner<Square> in each return. The converting constructor template is the candidate (OVL-14), and deduction gives U = Square (DED-1). The local in from_local is an xvalue in the return (CPY-11).
template <class T> class Owner {
public:
  T* held;
  explicit Owner(T* p) : held(p) {}
  Owner(Owner&& from) noexcept : held(from.held) { from.held = nullptr; }
  template <class U> Owner(Owner<U>&& from) noexcept : held(from.held) { from.held = nullptr; }
  ~Owner() { delete held; }
};
struct Shape { virtual ~Shape() = default; virtual int sides() const = 0; };
struct Square : Shape { int sides() const override { return 4; } };
Owner<Square> make_square() { return Owner<Square>(new Square); }
Owner<Shape> from_prvalue() { return make_square(); }
Owner<Shape> from_local() {
  Owner<Square> local(new Square);
  return local;
}
int main() {
  Owner<Shape> a = from_prvalue();
  Owner<Shape> b = from_local();
  return a.held->sides() + b.held->sides() == 8 ? 0 : 1;
}
