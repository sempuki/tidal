struct Base { virtual ~Base() {} virtual int v() const { return 1; } };
struct Derived : Base { int v() const override { return 2; } };
template <class T> struct Owner {
  T* p;
  explicit Owner(T* q) : p(q) {}
  Owner(Owner&& o) : p(o.p) { o.p = nullptr; }
  template <class U> Owner(Owner<U>&& o) : p(o.p) { o.p = nullptr; }
  Owner& operator=(Owner&& o) { if (this != &o) { delete p; p = o.p; o.p = nullptr; } return *this; }
  ~Owner() { delete p; }
};
int main() {
  Owner<Base> b(new Base);
  b = Owner<Derived>(new Derived);
  return b.p->v() == 2 ? 0 : 1;
}
