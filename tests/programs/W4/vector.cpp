// tidal-rules: NEW-19 NEW-26 NEW-28 LSUP-5 LSUP-7 STG-5 STG-7 STG-8 DTOR-6 NEW-9 ADD-6 CPY-8 OBJ-2
// tidal-expect: run
// tidal-note: Write suite W4, program vector.cpp (reports/clean-room/W4/report.md). The report has no construct-to-rule notes, so the rules are the ones the program's comments name.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// W4: a dynamic array that grows and keeps its elements in raw storage.
// Rules: NEW-19, NEW-26, NEW-28, LSUP-5, LSUP-7, STG-5, STG-7, STG-8,
// DTOR-6, NEW-9, ADD-6, CPY-8, OBJ-2.

namespace std {
using size_t = decltype(sizeof 0);
enum class align_val_t : size_t {};        // LSUP-5
}

inline void* operator new(std::size_t, void* at) noexcept { return at; }   // LSUP-7
inline void operator delete(void*, void*) noexcept {}

template <class T>
class Vector {
  T* data = nullptr;                       // raw storage; elements 0..count-1 live
  std::size_t count = 0;
  std::size_t cap = 0;

  static T* allocate(std::size_t n) {
    // NEW-26: the aligned form, so an over-aligned T also gets its alignment.
    void* raw = ::operator new(n * sizeof(T), std::align_val_t(alignof(T)));
    return static_cast<T*>(raw);          // STG-8: the call creates a T array
  }

  static void deallocate(T* p) {
    ::operator delete(p, std::align_val_t(alignof(T)));
  }

  void grow() {
    std::size_t new_cap = cap == 0 ? 2 : cap * 2;
    T* fresh = allocate(new_cap);
    for (std::size_t i = 0; i < count; ++i) {
      new (fresh + i) T(static_cast<T&&>(data[i]));   // move into new storage
      data[i].~T();                                    // DTOR-6
    }
    if (data != nullptr) deallocate(data);
    data = fresh;
    cap = new_cap;
  }

public:
  Vector() = default;
  Vector(const Vector& other) : data(nullptr), count(0), cap(0) {
    if (other.count == 0) return;
    data = allocate(other.count);
    cap = other.count;
    for (; count < other.count; ++count)
      new (data + count) T(other.data[count]);
  }
  Vector& operator=(const Vector&) = delete;
  ~Vector() {
    clear();
    if (data != nullptr) deallocate(data);
  }

  void push_back(const T& v) {
    if (count == cap) grow();
    new (data + count) T(v);               // STG-5
    ++count;
  }

  template <class... Args>
  T& emplace_back(Args&&... args) {
    if (count == cap) grow();
    T* p = new (data + count) T(static_cast<Args&&>(args)...);
    ++count;
    return *p;
  }

  void pop_back() {
    --count;
    data[count].~T();                      // the storage stays; the element ends
  }

  void clear() {
    while (count > 0) pop_back();          // last element first
  }

  T& operator[](std::size_t i) { return data[i]; }
  const T& operator[](std::size_t i) const { return data[i]; }
  std::size_t size() const { return count; }
  std::size_t capacity() const { return cap; }
  T* begin() { return data; }
  T* end() { return data + count; }
};

int live = 0;                              // number of Item objects alive

struct Item {
  int value;
  int* payload;                            // owns one int
  explicit Item(int v) : value(v), payload(new int(v * 2)) { ++live; }
  Item(const Item& o) : value(o.value), payload(new int(*o.payload)) { ++live; }
  Item(Item&& o) : value(o.value), payload(o.payload) {
    o.payload = nullptr;
    ++live;
  }
  Item& operator=(const Item&) = delete;
  ~Item() {
    delete payload;
    --live;
  }
};

struct alignas(64) Wide {
  int v;
};

int main() {
  {
    Vector<int> ints;
    for (int i = 0; i < 100; ++i) ints.push_back(i);
    if (ints.size() != 100) return 1;
    if (ints.capacity() < 100) return 2;
    int sum = 0;
    for (int x : ints) sum += x;           // begin and end
    if (sum != 4950) return 3;
    ints.pop_back();
    if (ints.size() != 99 || ints[98] != 98) return 4;
  }

  {
    Vector<Item> items;
    for (int i = 1; i <= 10; ++i) items.emplace_back(i);
    if (live != 10) return 5;
    if (items[9].value != 10 || *items[9].payload != 20) return 6;
    Vector<Item> copy(items);
    if (live != 20) return 7;
    *copy[0].payload = 99;
    if (*items[0].payload != 2) return 8;  // the copy owns its own int
    items.clear();
    if (live != 10 || items.size() != 0) return 9;
    items.push_back(copy[0]);
    if (*items[0].payload != 99) return 10;
  }
  if (live != 0) return 11;                // every Item ended

  {
    Vector<Wide> wides;
    for (int i = 0; i < 5; ++i) wides.push_back(Wide{i});
    for (std::size_t i = 0; i < wides.size(); ++i) {
      if (reinterpret_cast<std::size_t>(&wides[i]) % 64 != 0) return 12;
      if (wides[i].v != static_cast<int>(i)) return 13;
    }
  }
  return 0;
}
