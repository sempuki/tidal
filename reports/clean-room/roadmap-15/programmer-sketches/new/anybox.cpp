// A type-erased value with a small buffer: Any keeps a value of any
// copyable type, in place when it fits and on the heap otherwise, and
// any_cast gives it back by type. The operations come from a table of
// function pointers per type, and a type's identity is the address of
// a variable template. Any keeps the pointer that placement new gives,
// because a cast of the buffer's address reaches only its bytes.
// No standard header.

using size_t = decltype(sizeof 0);

inline void* operator new(size_t, void* at) noexcept { return at; }

namespace ab {

template <class T> struct remove_ref { using type = T; };
template <class T> struct remove_ref<T&> { using type = T; };
template <class T> struct remove_ref<T&&> { using type = T; };
template <class T> using remove_ref_t = typename remove_ref<T>::type;

template <class T> struct remove_cv { using type = T; };
template <class T> struct remove_cv<const T> { using type = T; };
template <class T> using decay_t = typename remove_cv<remove_ref_t<T>>::type;

template <class T>
constexpr remove_ref_t<T>&& move(T&& t) noexcept {
  return static_cast<remove_ref_t<T>&&>(t);
}
template <class T>
constexpr T&& forward(remove_ref_t<T>& t) noexcept {
  return static_cast<T&&>(t);
}

template <class A, class B> inline constexpr bool same = false;
template <class A> inline constexpr bool same<A, A> = true;

template <class T> inline constexpr char type_tag = 0;
template <class T> constexpr const void* type_id() { return &type_tag<T>; }

// Each operation takes the object pointer, and copy and move give the
// pointer to the new object.
struct VTable {
  void (*destroy)(void* obj) noexcept;
  void* (*copy)(void* buf, const void* obj);
  void* (*move)(void* buf, void* obj) noexcept;
  const void* type;
};

inline constexpr size_t buffer_size = 3 * sizeof(void*);
inline constexpr size_t buffer_align = alignof(long double);

template <class T>
inline constexpr bool fits = sizeof(T) <= buffer_size && alignof(T) <= buffer_align &&
                             noexcept(T(static_cast<T&&>(*static_cast<T*>(nullptr))));

template <class T, bool Small = fits<T>>
struct Ops {
  static void destroy(void* obj) noexcept { static_cast<T*>(obj)->~T(); }
  static void* copy(void* buf, const void* obj) {
    return ::new (buf) T(*static_cast<const T*>(obj));
  }
  static void* move(void* buf, void* obj) noexcept {
    T* from = static_cast<T*>(obj);
    T* to = ::new (buf) T(ab::move(*from));
    from->~T();
    return to;
  }
};

template <class T>
struct Ops<T, false> {
  static void destroy(void* obj) noexcept { delete static_cast<T*>(obj); }
  static void* copy(void*, const void* obj) { return new T(*static_cast<const T*>(obj)); }
  static void* move(void*, void* obj) noexcept { return obj; }
};

template <class T>
inline constexpr VTable table = {&Ops<T>::destroy, &Ops<T>::copy, &Ops<T>::move, type_id<T>()};

class Any {
 public:
  Any() noexcept = default;

  template <class V>
    requires(!same<decay_t<V>, Any>)
  Any(V&& v) {
    emplace<decay_t<V>>(forward<V>(v));
  }

  Any(const Any& other) : vt_(other.vt_) {
    if (vt_) obj_ = vt_->copy(buf_, other.obj_);
  }
  Any(Any&& other) noexcept : vt_(other.vt_) {
    if (vt_) obj_ = vt_->move(buf_, other.obj_);
    other.vt_ = nullptr;
    other.obj_ = nullptr;
  }
  Any& operator=(Any other) noexcept {
    reset();
    if (other.vt_) {
      obj_ = other.vt_->move(buf_, other.obj_);
      vt_ = other.vt_;
      other.vt_ = nullptr;
      other.obj_ = nullptr;
    }
    return *this;
  }
  ~Any() { reset(); }

  template <class T, class... A>
  T& emplace(A&&... args) {
    reset();
    T* p;
    if constexpr (fits<T>) {
      p = ::new (buf_) T(forward<A>(args)...);
    } else {
      p = new T(forward<A>(args)...);
    }
    obj_ = p;
    vt_ = &table<T>;
    return *p;
  }

  void reset() noexcept {
    if (vt_) {
      vt_->destroy(obj_);
      vt_ = nullptr;
      obj_ = nullptr;
    }
  }
  bool has_value() const noexcept { return vt_ != nullptr; }
  const void* type() const noexcept { return vt_ ? vt_->type : nullptr; }

  template <class T> friend T* any_cast(Any* a) noexcept;

 private:
  alignas(buffer_align) unsigned char buf_[buffer_size];
  void* obj_ = nullptr;
  const VTable* vt_ = nullptr;
};

template <class T>
T* any_cast(Any* a) noexcept {
  if (a && a->vt_ == &table<T>) return static_cast<T*>(a->obj_);
  return nullptr;
}
template <class T>
const T* any_cast(const Any* a) noexcept {
  return any_cast<T>(const_cast<Any*>(a));
}

}  // namespace ab

// A type that counts its live objects, to check that Any leaks nothing.
struct Counted {
  static inline int live = 0;
  int v;
  Counted(int x) : v(x) { ++live; }
  Counted(const Counted& o) : v(o.v) { ++live; }
  Counted(Counted&& o) noexcept : v(o.v) { ++live; }
  ~Counted() { --live; }
};

// Too big for the buffer.
struct Big {
  static inline int live = 0;
  long long a[8];
  Big(long long x) : a{x, x + 1, x + 2, x + 3, x + 4, x + 5, x + 6, x + 7} { ++live; }
  Big(const Big& o) : Big(o.a[0]) {}
  ~Big() { --live; }
  long long sum() const {
    long long s = 0;
    for (long long x : a) s += x;
    return s;
  }
};

static_assert(ab::fits<int>);
static_assert(ab::fits<Counted>);
static_assert(!ab::fits<Big>);
static_assert(ab::type_id<int>() != ab::type_id<long>());

int run() {
  using ab::Any;
  using ab::any_cast;
  {
    Any a = 42;
    if (!a.has_value() || a.type() != ab::type_id<int>()) return 1;
    if (*any_cast<int>(&a) != 42 || any_cast<long>(&a)) return 2;

    Any b = Counted{7};
    Any c = b;
    if (Counted::live != 2) return 3;
    any_cast<Counted>(&c)->v = 8;
    if (any_cast<Counted>(&b)->v != 7) return 4;

    Any d = Big{10};
    Any e = d;
    if (Big::live != 2 || any_cast<Big>(&e)->sum() != 108) return 5;
    Any f = ab::move(d);
    if (d.has_value() || Big::live != 2) return 6;

    a = b;
    if (any_cast<int>(&a) || any_cast<Counted>(&a)->v != 7 || Counted::live != 3) return 7;
    a = f;
    if (Counted::live != 2 || Big::live != 3) return 8;
    a.emplace<Counted>(99);
    if (any_cast<Counted>(&a)->v != 99 || Big::live != 2) return 9;

    const Any& ca = a;
    if (any_cast<Counted>(&ca)->v != 99) return 10;
  }
  if (Counted::live != 0 || Big::live != 0) return 11;

  // A small vector of mixed values.
  ab::Any items[4] = {1, Counted{2}, Big{3}, 4.5};
  long long total = 0;
  for (ab::Any& x : items) {
    if (int* i = any_cast<int>(&x)) total += *i;
    else if (Counted* c = any_cast<Counted>(&x)) total += c->v;
    else if (Big* b = any_cast<Big>(&x)) total += b->a[0];
    else if (double* d = any_cast<double>(&x)) total += static_cast<long long>(*d * 2);
  }
  return total == 1 + 2 + 3 + 9 ? 0 : 12;
}

int main() {
  int r = run();
  if (r) return r;
  return Counted::live == 0 && Big::live == 0 ? 0 : 13;
}
