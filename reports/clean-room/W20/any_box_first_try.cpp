// Box: holds one copyable value of any type, with no run-time type information.
// Rules used: ODR-12 and LNK (one object for each specialization of a
// variable template, so its address is a per-type tag), CONV (T* to void*
// and back with static_cast), NEW (new and delete of the held value),
// CPY-P2 (the class writes copy, move, and destructor), CPT-8 and CPT-31
// (requires clause: the type must be copyable), LAM-10 (not used),
// CE-P1 (the tag comparison in static_assert).

template <class T>
inline constexpr char type_tag = 0;

struct BoxOps {
  const void* tag;
  void* (*copy)(const void*);
  void (*destroy)(void*);
};

template <class T>
struct OpsFor {
  static void* copy(const void* p) { return new T(*static_cast<const T*>(p)); }
  static void destroy(void* p) { delete static_cast<T*>(p); }
  static constexpr BoxOps ops{&type_tag<T>, &copy, &destroy};
};

class Box {
  const BoxOps* ops = nullptr;
  void* held = nullptr;
public:
  Box() = default;
  template <class T>
    requires requires(const T& t) { T(t); }
  Box(T value) : ops(&OpsFor<T>::ops), held(new T(static_cast<T&&>(value))) {}
  Box(const Box& o) : ops(o.ops), held(o.ops ? o.ops->copy(o.held) : nullptr) {}
  Box(Box&& o) : ops(o.ops), held(o.held) { o.ops = nullptr; o.held = nullptr; }
  Box& operator=(const Box& o) {
    if (this != &o) {
      Box copy(o);
      swap(copy);
    }
    return *this;
  }
  Box& operator=(Box&& o) {
    Box moved(static_cast<Box&&>(o));
    swap(moved);
    return *this;
  }
  ~Box() { if (ops) ops->destroy(held); }

  void swap(Box& o) {
    const BoxOps* t = ops; ops = o.ops; o.ops = t;
    void* h = held; held = o.held; o.held = h;
  }
  bool has_value() const { return ops != nullptr; }
  template <class T>
  bool holds() const { return ops && ops->tag == &type_tag<T>; }
  template <class T>
  T* get_if() { return holds<T>() ? static_cast<T*>(held) : nullptr; }
  template <class T>
  const T* get_if() const { return holds<T>() ? static_cast<const T*>(held) : nullptr; }
};

static_assert(&type_tag<int> != static_cast<const void*>(&type_tag<long>));
static_assert(&type_tag<int> == &type_tag<int>);

struct Tracked {
  static int live;
  int id;
  Tracked(int i) : id(i) { ++live; }
  Tracked(const Tracked& o) : id(o.id) { ++live; }
  ~Tracked() { --live; }
};
int Tracked::live = 0;

int run() {
  {
    Box a = 42;
    Box b = Tracked(7);
    if (!a.holds<int>() || a.holds<long>()) return 1;
    if (*a.get_if<int>() != 42 || a.get_if<double>() != nullptr) return 2;
    if (b.get_if<Tracked>()->id != 7 || Tracked::live != 1) return 3;
    Box c = b;                       // copy
    if (Tracked::live != 2 || c.get_if<Tracked>() == b.get_if<Tracked>()) return 4;
    c = a;                           // copy assignment to another type
    if (Tracked::live != 1 || *c.get_if<int>() != 42) return 5;
    Box d = static_cast<Box&&>(b);   // move
    if (b.has_value() || d.get_if<Tracked>()->id != 7) return 6;
    a = static_cast<Box&&>(d);       // move assignment
    if (!a.holds<Tracked>() || Tracked::live != 1) return 7;
    const Box e = 2.5;
    if (*e.get_if<double>() != 2.5) return 8;
    Box empty;
    if (empty.has_value() || empty.get_if<int>() != nullptr) return 9;
  }
  if (Tracked::live != 0) return 10;
  return 0;
}

int main() { return run(); }
