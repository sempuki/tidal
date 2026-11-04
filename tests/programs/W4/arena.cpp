// tidal-rules: STG-4 STG-5 STG-6 STG-17 ALN-1 ALN-4 UN-16 UN-12 LSUP-7 NEW-10 NEW-33 DTOR-6 CAST-1 CAST-6 CAST-16 ADD-6 PFX-7 LSUP-4 NEW-11
// tidal-expect: run
// tidal-note: Write suite W4, program arena.cpp (reports/clean-room/W4/report.md). The report has no construct-to-rule notes, so the rules are the ones the program's comments name.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// W4: a bump-pointer arena that hands out aligned storage for objects of
// different types.
// Rules: STG-4, STG-5, STG-6, STG-17, ALN-1, ALN-4, UN-16, UN-12, LSUP-7,
// NEW-10, NEW-33, DTOR-6, CAST-1, CAST-6, CAST-16, ADD-6, PFX-7.

namespace std {
using size_t = decltype(sizeof 0);         // LSUP-4
}

// The non-allocating placement form (LSUP-7).
inline void* operator new(std::size_t, void* at) noexcept { return at; }
inline void* operator new[](std::size_t, void* at) noexcept { return at; }
inline void operator delete(void*, void*) noexcept {}
inline void operator delete[](void*, void*) noexcept {}

class Arena {
  static constexpr std::size_t capacity = 1024;
  static constexpr std::size_t max_align = 64;

  // A byte array hosts the objects (STG-4). Its start is aligned to 64, so
  // an offset that is a multiple of an alignment up to 64 gives an aligned
  // address.
  alignas(max_align) unsigned char buffer[capacity];
  std::size_t used = 0;

  // Each object with a destructor gets a record. The arena runs the
  // destructors in reverse order when it is reset.
  struct Cleanup {
    void (*destroy)(void*, std::size_t);
    void* object;
    std::size_t count;                     // 1 for one object, n for an array
    Cleanup* next;
  };
  Cleanup* cleanups = nullptr;

  template <class T>
  static void destroy_all(void* p, std::size_t n) {
    T* first = static_cast<T*>(p);         // CAST-1
    while (n > 0) {
      --n;
      first[n].~T();                       // DTOR-6, last element first
    }
  }

  bool remember(void (*destroy)(void*, std::size_t), void* object,
                std::size_t count) {
    void* rec = bump(sizeof(Cleanup), alignof(Cleanup));
    if (rec == nullptr) return false;
    cleanups = new (rec) Cleanup{destroy, object, count, cleanups};
    return true;
  }

  // Raw bytes, or nullptr if the arena is full.
  void* bump(std::size_t size, std::size_t align) {
    std::size_t start = (used + align - 1) & ~(align - 1);
    if (align > max_align || start > capacity || size > capacity - start)
      return nullptr;
    used = start + size;
    return buffer + start;                 // ADD-6: a position in the byte array
  }

public:
  Arena() = default;
  Arena(const Arena&) = delete;
  Arena& operator=(const Arena&) = delete;
  ~Arena() { reset(); }

  template <class T, class... Args>
  T* make(Args&&... args) {
    void* place = bump(sizeof(T), alignof(T));   // UN-12, UN-16
    if (place == nullptr) return nullptr;
    T* object = new (place) T(static_cast<Args&&>(args)...);  // STG-5
    if (!remember(&destroy_all<T>, object, 1)) {
      object->~T();
      return nullptr;
    }
    return object;
  }

  // An array of n value-initialized objects of T. Placement new creates the
  // array, also in bytes that held other objects before a reset (STG-5,
  // NEW-33: no extra storage, the array starts at place).
  template <class T>
  T* make_array(std::size_t n) {
    if (n == 0 || n > capacity / sizeof(T)) return nullptr;
    void* place = bump(n * sizeof(T), alignof(T));
    if (place == nullptr) return nullptr;
    T* first = new (place) T[n]();
    if (!remember(&destroy_all<T>, first, n)) {
      destroy_all<T>(first, n);
      return nullptr;
    }
    return first;
  }

  void reset() {
    for (Cleanup* c = cleanups; c != nullptr; c = c->next)
      c->destroy(c->object, c->count);               // PFX-7
    cleanups = nullptr;
    used = 0;
  }

  std::size_t bytes_used() const { return used; }
};

bool aligned(const void* p, std::size_t align) {
  // CAST-6 and CAST-16: on the matrix targets the integer is the address.
  return reinterpret_cast<std::size_t>(p) % align == 0;
}

int destroyed = 0;

struct Point {
  int x, y;
  Point(int a, int b) : x(a), y(b) {}
};

struct alignas(32) Block {                 // ALN-1: alignment 32
  double values[3];
};

struct Tracked {
  int id;
  Tracked() : id(0) {}                     // NEW-11: needed for an array
  explicit Tracked(int i) : id(i) {}
  ~Tracked() { destroyed = destroyed * 10 + id; }
};

int main() {
  Arena arena;

  char* c = arena.make<char>('a');
  Point* p = arena.make<Point>(3, 4);
  Block* b = arena.make<Block>();
  double* d = arena.make<double>(2.5);
  Tracked* t1 = arena.make<Tracked>(1);
  Tracked* t2 = arena.make<Tracked>(2);

  if (!c || !p || !b || !d || !t1 || !t2) return 1;
  if (*c != 'a' || p->x != 3 || p->y != 4 || *d != 2.5) return 2;
  if (!aligned(p, alignof(Point))) return 3;
  if (!aligned(b, 32)) return 4;
  if (!aligned(d, alignof(double))) return 5;
  b->values[2] = 1.5;
  if (b->values[2] != 1.5) return 6;

  int* numbers = arena.make_array<int>(10);
  if (numbers == nullptr || !aligned(numbers, alignof(int))) return 7;
  for (int i = 0; i < 10; ++i) numbers[i] = i * i;
  if (numbers[9] != 81 || numbers[0] != 0) return 8;

  // The arena refuses a request that does not fit.
  if (arena.make_array<char>(2000) != nullptr) return 9;

  arena.reset();                           // runs ~Tracked for 2, then 1
  if (destroyed != 21) return 10;
  if (arena.bytes_used() != 0) return 11;

  // The storage is reused for new objects (STG-6).
  Point* q = arena.make<Point>(7, 8);
  if (q == nullptr || q->x + q->y != 15) return 12;

  // An array of objects with destructors, then ints in bytes that held a
  // Point and its cleanup record before the next reset.
  destroyed = 0;
  if (arena.make_array<Tracked>(0) != nullptr) return 13;
  Tracked* row = arena.make_array<Tracked>(3);
  if (row == nullptr) return 13;
  for (int i = 0; i < 3; ++i) row[i].id = i + 1;
  arena.reset();                           // elements 3, 2, 1, then the Point
  if (destroyed != 321) return 13;
  int* reused = arena.make_array<int>(4);
  if (reused == nullptr || reused[3] != 0) return 14;
  reused[3] = 9;
  if (reused[3] != 9) return 15;
  return 0;
}
