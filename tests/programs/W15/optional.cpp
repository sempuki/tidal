// tidal-rules: LSUP-4 LSUP-7 LSUP-10 TR-2 TR-5 TR-10 CE-1 STG-4 STG-5 STG-6 ALN-1 STG-9 STG-16 STG-19 DTOR-6 DTOR-7 STG-22
// tidal-expect: run
// tidal-note: Write suite W15, program optional.cpp (reports/clean-room/W15/report.md). The rules are the ones the report's notes made before writing give for this program.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// W15 exercise 5: Optional<T> with the T in raw storage inside the object.
// Written from the Tidal specification by a clean-room reader.
//
// Rules used:
//   STG-4            an aligned array of unsigned char hosts the T
//   ALN-1            alignas(T) on the byte array
//   LSUP-7, NEW-33   the non-allocating placement form, declared here,
//                    returns its argument; the new T is at that address
//   STG-5            placement new creates the T; the result points to it
//   STG-16, STG-19   the byte array and the T are not pointer-
//                    interconvertible, and the launder operation is library
//                    referenced, so the class keeps the pointer that
//                    placement new returned
//   STG-6, STG-9     a new T in the same bytes ends the old one with no
//                    destructor call, so reset() runs the destructor first
//   DTOR-6           p->~T() ends the T and keeps the storage
//   DTOR-7           no second destruction: the pointer is cleared
//   CPY-P1, CPY-P2   the implicit copy would copy the pointer and point
//                    into the other object's bytes, so the class writes
//                    its own copy and move operations
//   PACK-22, PACK-23 emplace forwards its arguments

namespace std {
template <class T, class... Args>
constexpr T* construct_at(T* at, Args&&... args) {
  return ::new (static_cast<void*>(at)) T(static_cast<Args&&>(args)...);
}
}  // namespace std
inline void* operator new(decltype(sizeof 0), void* at) noexcept { return at; }

template <class T>
class Optional {
  alignas(T) unsigned char storage[sizeof(T)];
  T* object = nullptr;  // points into storage while a T exists

  template <class... Args>
  void build(Args&&... args) {
    object = ::new (static_cast<void*>(storage)) T(static_cast<Args&&>(args)...);
  }

 public:
  Optional() noexcept {}
  Optional(const T& v) { build(v); }
  Optional(T&& v) { build(static_cast<T&&>(v)); }

  Optional(const Optional& other) {
    if (other.object) build(*other.object);
  }
  Optional(Optional&& other) {
    if (other.object) build(static_cast<T&&>(*other.object));
  }

  Optional& operator=(const Optional& other) {
    if (this == &other) return *this;
    if (!other.object) {
      reset();
    } else if (object) {
      *object = *other.object;  // both hold a T: assign the T
    } else {
      build(*other.object);
    }
    return *this;
  }
  Optional& operator=(Optional&& other) {
    if (this == &other) return *this;
    if (!other.object) {
      reset();
    } else if (object) {
      *object = static_cast<T&&>(*other.object);
    } else {
      build(static_cast<T&&>(*other.object));
    }
    return *this;
  }

  ~Optional() { reset(); }

  void reset() {
    if (object) {
      object->~T();
      object = nullptr;
    }
  }

  // Ends any T first, then builds a new one in the same bytes.
  template <class... Args>
  T& emplace(Args&&... args) {
    reset();
    object = std::construct_at(reinterpret_cast<T*>(storage),
                               static_cast<Args&&>(args)...);
    return *object;
  }

  bool has_value() const { return object != nullptr; }
  T& operator*() { return *object; }
  const T& operator*() const { return *object; }
  T* operator->() { return object; }
};

// Counts constructions and destructions of each kind.
struct Tracked {
  static int made;       // every constructor
  static int ended;      // destructor
  static int assigned;   // copy and move assignment
  int value;

  explicit Tracked(int v) : value(v) { ++made; }
  Tracked(int a, int b) : value(a * 10 + b) { ++made; }
  Tracked(const Tracked& o) : value(o.value) { ++made; }
  Tracked(Tracked&& o) : value(o.value) { ++made; o.value = -1; }
  Tracked& operator=(const Tracked& o) { value = o.value; ++assigned; return *this; }
  Tracked& operator=(Tracked&& o) { value = o.value; o.value = -1; ++assigned; return *this; }
  ~Tracked() { ++ended; }

  static int live() { return made - ended; }
};
int Tracked::made = 0;
int Tracked::ended = 0;
int Tracked::assigned = 0;

// A type with no default constructor and a non-trivial destructor that
// would leak if the destructor did not run.
struct Buffer {
  int* data;
  explicit Buffer(int n) : data(new int[n]{}) {}
  Buffer(const Buffer&) = delete;
  Buffer& operator=(const Buffer&) = delete;
  ~Buffer() { delete[] data; }
};

int check() {
  // Empty: nothing is built.
  {
    Optional<Tracked> o;
    if (o.has_value()) return 10;
    if (Tracked::made != 0) return 11;
  }
  if (Tracked::ended != 0) return 12;

  // emplace builds one T; destruction of the Optional ends it.
  {
    Optional<Tracked> o;
    o.emplace(4, 2);
    if (!o.has_value() || o->value != 42) return 20;
    if (Tracked::live() != 1) return 21;
  }
  if (Tracked::live() != 0 || Tracked::made != 1) return 22;

  // reset ends the T at once, and a second reset does nothing.
  {
    Optional<Tracked> o(Tracked(5));  // temporary + move into storage
    if (Tracked::live() != 1) return 30;
    o.reset();
    if (Tracked::live() != 0) return 31;
    o.reset();
    if (Tracked::live() != 0) return 32;
  }
  if (Tracked::live() != 0) return 33;

  // emplace on a full Optional ends the old T first.
  {
    Optional<Tracked> o;
    o.emplace(1);
    int ended_before = Tracked::ended;
    o.emplace(2);
    if (Tracked::ended != ended_before + 1) return 40;
    if (Tracked::live() != 1 || o->value != 2) return 41;
  }
  if (Tracked::live() != 0) return 42;

  // Copy assignment: full to empty builds, full to full assigns, empty to
  // full ends.
  {
    Optional<Tracked> a;
    a.emplace(7);
    Optional<Tracked> b;
    b = a;                                 // builds a copy
    if (Tracked::live() != 2 || b->value != 7) return 50;
    int assigned_before = Tracked::assigned;
    a.emplace(8);
    b = a;                                 // T's assignment, no new T
    if (Tracked::assigned != assigned_before + 1 || b->value != 8) return 51;
    if (Tracked::live() != 2) return 52;
    Optional<Tracked> none;
    b = none;                              // ends b's T
    if (b.has_value() || Tracked::live() != 1) return 53;
    // Self-assignment through a reference: Clang warns on a plain `b = b`
    // (-Wself-assign-overloaded).
    Optional<Tracked>& same_b = b;
    Optional<Tracked>& same_a = a;
    b = same_b;                            // empty to itself
    a = same_a;                            // full to itself
    if (Tracked::live() != 1 || a->value != 8) return 54;
  }
  if (Tracked::live() != 0) return 55;

  // Copy and move construction, and move assignment.
  {
    Optional<Tracked> a;
    a.emplace(3);
    Optional<Tracked> b(a);
    Optional<Tracked> c(static_cast<Optional<Tracked>&&>(a));
    if (Tracked::live() != 3) return 60;
    if (b->value != 3 || c->value != 3 || a->value != -1) return 61;
    Optional<Tracked> d;
    d = static_cast<Optional<Tracked>&&>(b);
    if (Tracked::live() != 4 || d->value != 3) return 62;
  }
  if (Tracked::live() != 0) return 63;

  // A type with no default constructor. Under the address sanitizer, a
  // missing destructor call would show as a leak.
  {
    Optional<Buffer> o;
    o.emplace(16);
    o->data[15] = 1;
    o.emplace(8);
    o.reset();
    o.emplace(4);
  }

  // Optional<int>: the destructor call names a scalar type (DTOR-6).
  {
    Optional<int> i(5);
    Optional<int> j;
    j = i;
    if (*j != 5) return 70;
    i.reset();
    if (i.has_value()) return 71;
  }

  return 0;
}

int main() {
  int r = check();
  if (r != 0) return r;
  // Every constructed Tracked was destroyed exactly once.
  return Tracked::made == Tracked::ended ? 0 : 99;
}
