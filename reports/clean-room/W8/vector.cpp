// A container that stays valid if an element constructor throws,
// written from the Tidal specification.
// Rules used: NEW-19, NEW-38, STG-5, STG-8, LSUP-4, LSUP-7, NEW-10,
// DTOR-6, UNW-1, UNW-2, UNW-3, TRY-8, THR-5, CPY-9, CPY-2, CPY-3.

namespace std {
using size_t = decltype(sizeof 0);
}
inline void* operator new(std::size_t, void* at) noexcept { return at; }
inline void operator delete(void*, void*) noexcept {}

int alive = 0;          // Item objects that exist now
int budget = -1;        // constructions left before one throws; -1 means never

struct Failure { int code; };

struct Item {
  int value;
  explicit Item(int v) : value(v) {
    if (budget == 0) throw Failure{v};
    if (budget > 0) --budget;
    ++alive;
  }
  Item(const Item& o) : value(o.value) {
    if (budget == 0) throw Failure{o.value};
    if (budget > 0) --budget;
    ++alive;
  }
  Item& operator=(const Item&) = default;
  ~Item() { --alive; }
};

template <class T>
class Vector {
  T* data = nullptr;    // storage from ::operator new; elements 0..size-1 exist
  int count = 0;
  int capacity = 0;

  static T* allocate(int n) {
    return static_cast<T*>(::operator new(sizeof(T) * static_cast<std::size_t>(n)));
  }
  static void destroy_range(T* p, int n) {
    while (n > 0) {
      --n;
      p[n].~T();        // ends the object, keeps the storage (DTOR-6)
    }
  }

public:
  Vector() = default;
  Vector(const Vector&) = delete;
  Vector& operator=(const Vector&) = delete;

  ~Vector() {
    destroy_range(data, count);
    ::operator delete(data);
  }

  int size() const { return count; }
  int room() const { return capacity; }
  T& operator[](int i) { return data[i]; }

  // Adds a copy of value at the end. If a constructor throws, the vector
  // has the same elements and the same storage as before.
  void push_back(const T& value) {
    if (count < capacity) {
      new (data + count) T(value);   // a throw here changes nothing
      ++count;
      return;
    }
    int bigger = capacity == 0 ? 2 : capacity * 2;
    T* fresh = allocate(bigger);
    int built = 0;
    bool added = false;
    try {
      // The new element first: value can be an element of this vector.
      new (fresh + count) T(value);
      added = true;
      for (; built < count; ++built) new (fresh + built) T(data[built]);
    } catch (...) {
      destroy_range(fresh, built);
      if (added) fresh[count].~T();
      ::operator delete(fresh);
      throw;                          // the same exception goes on (TRY-8)
    }
    destroy_range(data, count);
    ::operator delete(data);
    data = fresh;
    capacity = bigger;
    ++count;
  }
};

int main() {
  {
    Vector<Item> v;
    v.push_back(Item(1));
    v.push_back(Item(2));
    if (v.size() != 2 || v.room() != 2) return 1;
    if (alive != 2) return 2;

    // The new element throws while the vector grows.
    budget = 0;
    bool caught = false;
    try {
      v.push_back(v[0]);
    } catch (Failure& f) {
      caught = f.code == 1;
    }
    if (!caught) return 3;
    if (v.size() != 2 || v.room() != 2) return 4;
    if (v[0].value != 1 || v[1].value != 2) return 5;
    if (alive != 2) return 6;

    // The new element succeeds and the copy of element 1 throws.
    budget = 2;
    caught = false;
    try {
      v.push_back(v[1]);
    } catch (Failure& f) {
      caught = f.code == 2;
    }
    if (!caught) return 7;
    if (v.size() != 2 || v.room() != 2) return 8;
    if (alive != 2) return 9;

    // Growth works when nothing throws.
    budget = -1;
    v.push_back(v[1]);
    if (v.size() != 3 || v.room() != 4) return 10;
    if (v[2].value != 2 || alive != 3) return 11;

    // A throw with room to spare changes nothing.
    budget = 1;
    caught = false;
    try {
      Item extra(9);              // uses the last construction; the copy throws
      v.push_back(extra);
    } catch (Failure&) {
      caught = true;
    }
    if (!caught || v.size() != 3 || alive != 3) return 12;
    budget = -1;
  }
  if (alive != 0) return 13;
  return 0;
}
