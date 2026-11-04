template <class T> class Owner {
  T* ptr;
  template <class U> friend class Owner;
public:
  explicit Owner(T* p) : ptr(p) {}
  template <class U> Owner(Owner<U>&& o) : ptr(o.ptr) { o.ptr = nullptr; }
  ~Owner() { delete ptr; }
};
struct B { virtual ~B() = default; };
struct D : B {};
int main() { Owner<B> b = Owner<D>(new D); }
