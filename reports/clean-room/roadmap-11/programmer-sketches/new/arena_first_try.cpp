// A monotonic Arena over an aligned byte buffer: allocate with alignment,
// make<T>(args...) that builds in place and records a destructor in a
// record that also lives in the arena, a Mark that rewinds the arena and
// destroys what came after it in reverse order, a constructor that throws
// in make, an over-aligned type, and an ArenaVec<T> that grows by moving
// into a new block of the arena.
namespace std { using size_t = decltype(sizeof 0); }
inline void* operator new(std::size_t, void* p) noexcept { return p; }

template <class T> struct remove_ref { using type = T; };
template <class T> struct remove_ref<T&> { using type = T; };
template <class T> struct remove_ref<T&&> { using type = T; };
template <class T> constexpr T&& forward(typename remove_ref<T>::type& t) noexcept { return static_cast<T&&>(t); }
template <class T> constexpr typename remove_ref<T>::type&& move(T&& t) noexcept {
  return static_cast<typename remove_ref<T>::type&&>(t);
}

struct OutOfMemory {};

class Arena {
  struct Record {
    void (*destroy)(void*);
    void* obj;
    Record* prev;
  };
  alignas(32) unsigned char buf_[2048];
  std::size_t used_ = 0;
  Record* last_ = nullptr;

  template <class T> static void destroy(void* p) { static_cast<T*>(p)->~T(); }

public:
  struct Mark { std::size_t used; Record* last; };

  Arena() = default;
  Arena(const Arena&) = delete;
  Arena& operator=(const Arena&) = delete;
  ~Arena() { rewind(Mark{0, nullptr}); }

  void* allocate(std::size_t size, std::size_t align) {
    std::size_t start = (used_ + align - 1) & ~(align - 1);
    if (start + size > sizeof buf_) throw OutOfMemory{};
    used_ = start + size;
    return buf_ + start;
  }
  template <class T, class... A> T* make(A&&... args) {
    void* rec = allocate(sizeof(Record), alignof(Record));
    void* p = allocate(sizeof(T), alignof(T));
    T* obj = ::new (p) T(forward<A>(args)...);
    last_ = ::new (rec) Record{&destroy<T>, obj, last_};
    return obj;
  }
  template <class T> T* make_array(std::size_t n) {
    return ::new (allocate(n * sizeof(T), alignof(T))) T[n]();
  }
  Mark mark() const { return Mark{used_, last_}; }
  void rewind(Mark m) {
    while (last_ != m.last) {
      Record* r = last_;
      last_ = r->prev;
      r->destroy(r->obj);
    }
    used_ = m.used;
  }
  std::size_t used() const { return used_; }
  std::size_t offset(const void* p) const { return static_cast<const unsigned char*>(p) - buf_; }
};

template <class T> class ArenaVec {
  Arena& arena_;
  T* data_ = nullptr;
  std::size_t size_ = 0, cap_ = 0;
public:
  explicit ArenaVec(Arena& a) : arena_(a) {}
  ArenaVec(const ArenaVec&) = delete;
  ~ArenaVec() { for (std::size_t i = 0; i < size_; ++i) data_[i].~T(); }
  void push_back(T v) {
    if (size_ == cap_) {
      std::size_t cap = cap_ ? 2 * cap_ : 2;
      T* d = static_cast<T*>(arena_.allocate(cap * sizeof(T), alignof(T)));
      for (std::size_t i = 0; i < size_; ++i) {
        ::new (&d[i]) T(move(data_[i]));
        data_[i].~T();
      }
      data_ = d;
      cap_ = cap;
    }
    ::new (&data_[size_++]) T(move(v));
  }
  T& operator[](std::size_t i) { return data_[i]; }
  std::size_t size() const { return size_; }
  T* begin() { return data_; }
  T* end() { return data_ + size_; }
};

inline int live = 0;
inline int order[8];
inline int destroyed = 0;

struct Tracked {
  int id;
  explicit Tracked(int i) : id(i) { ++live; }
  Tracked(const Tracked& o) : id(o.id) { ++live; }
  Tracked(Tracked&& o) noexcept : id(o.id) { o.id = -1; ++live; }
  ~Tracked() { --live; if (id >= 0 && destroyed < 8) order[destroyed++] = id; }
};
struct Fails {
  Fails(int v) { if (v < 0) throw v; ++live; }
  ~Fails() { --live; }
};
struct alignas(32) Wide { double d[4]; };

int main() {
  int fails = 0;
  {
    Arena arena;
    Tracked* a = arena.make<Tracked>(1);
    Arena::Mark m = arena.mark();
    arena.make<Tracked>(2);
    arena.make<Tracked>(3);
    Wide* w = arena.make<Wide>();
    if (arena.offset(w) % 32 != 0) ++fails;
    if (live != 3) ++fails;
    arena.rewind(m);
    if (live != 1 || destroyed != 2 || order[0] != 3 || order[1] != 2) ++fails;
    Tracked* b = arena.make<Tracked>(4);
    if (b->id != 4 || a->id != 1) ++fails;

    try {
      arena.make<Fails>(-1);
      ++fails;
    } catch (int v) {
      if (v != -1) ++fails;
    }
    if (live != 2) ++fails;

    int* nums = arena.make_array<int>(4);
    for (int i = 0; i < 4; ++i) if (nums[i] != 0) ++fails;
    nums[3] = 9;

    ArenaVec<Tracked> vec(arena);
    for (int i = 10; i < 15; ++i) vec.push_back(Tracked(i));
    int sum = 0;
    for (Tracked& t : vec) sum += t.id;
    if (sum != 10 + 11 + 12 + 13 + 14 || vec.size() != 5) ++fails;
    if (live != 7) ++fails;

    try {
      arena.allocate(4096, 8);
      ++fails;
    } catch (OutOfMemory) {
    }
  }
  if (live != 0) ++fails;
  return fails;
}
