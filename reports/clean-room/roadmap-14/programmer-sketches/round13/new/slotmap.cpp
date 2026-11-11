// SlotMap<T, N>: a fixed-capacity map from generational keys to values.
// Values sit packed in a union array in the object, so erase moves the
// last value into the hole. A key holds a slot index and a generation, so
// a key whose value was erased no longer finds anything, even after the
// slot is reused. emplace, get, erase, iteration over the packed values,
// and constant tests with a value type that counts its live objects.
namespace std {
using size_t = decltype(sizeof 0);
template <class T, class... Args>
constexpr T* construct_at(T* at, Args&&... args) {
  return ::new (static_cast<void*>(at)) T(static_cast<Args&&>(args)...);
}
}
inline void* operator new(std::size_t, void* at) noexcept { return at; }

template <class T> struct remove_ref { using type = T; };
template <class T> struct remove_ref<T&> { using type = T; };
template <class T> constexpr T&& forward(typename remove_ref<T>::type& t) noexcept { return static_cast<T&&>(t); }
template <class T> constexpr typename remove_ref<T>::type&& move(T&& t) noexcept {
  return static_cast<typename remove_ref<T>::type&&>(t);
}

struct Key {
  unsigned index = ~0u;
  unsigned gen = 0;
  constexpr bool operator==(const Key&) const = default;
};

template <class T, std::size_t N>
class SlotMap {
  union Cell {
    T value;
    constexpr Cell() {}
    constexpr ~Cell() {}
  };
  struct Slot {
    unsigned dense_or_next;  // the packed index when live, else the next free slot
    unsigned gen;
    bool live;
  };

  Cell cells_[N];
  unsigned owner_[N];  // the slot of each packed value
  Slot slots_[N];
  unsigned free_ = 0;
  std::size_t size_ = 0;

public:
  constexpr SlotMap() {
    for (unsigned i = 0; i < N; ++i) slots_[i] = {i + 1, 0, false};
  }
  SlotMap(const SlotMap&) = delete;
  SlotMap& operator=(const SlotMap&) = delete;
  constexpr ~SlotMap() {
    for (std::size_t i = 0; i < size_; ++i) cells_[i].value.~T();
  }

  constexpr std::size_t size() const { return size_; }
  constexpr bool full() const { return size_ == N; }

  template <class... A>
  constexpr Key emplace(A&&... a) {
    if (full()) return Key{};
    unsigned s = free_;
    Slot& slot = slots_[s];
    free_ = slot.dense_or_next;
    std::construct_at(&cells_[size_].value, forward<A>(a)...);
    owner_[size_] = s;
    slot.dense_or_next = static_cast<unsigned>(size_);
    slot.live = true;
    ++size_;
    return Key{s, slot.gen};
  }

  constexpr T* get(Key k) {
    if (k.index >= N) return nullptr;
    Slot& slot = slots_[k.index];
    if (!slot.live || slot.gen != k.gen) return nullptr;
    return &cells_[slot.dense_or_next].value;
  }
  constexpr bool contains(Key k) { return get(k) != nullptr; }

  constexpr bool erase(Key k) {
    if (!get(k)) return false;
    Slot& slot = slots_[k.index];
    unsigned hole = slot.dense_or_next;
    unsigned last = static_cast<unsigned>(size_ - 1);
    if (hole != last) {
      cells_[hole].value = move(cells_[last].value);
      owner_[hole] = owner_[last];
      slots_[owner_[hole]].dense_or_next = hole;
    }
    cells_[last].value.~T();
    --size_;
    slot.live = false;
    ++slot.gen;
    slot.dense_or_next = free_;
    free_ = k.index;
    return true;
  }

  // Iteration over the packed values, in packed order.
  class iterator {
    Cell* c_;

  public:
    constexpr explicit iterator(Cell* c) : c_(c) {}
    constexpr T& operator*() const { return c_->value; }
    constexpr iterator& operator++() { ++c_; return *this; }
    constexpr bool operator==(const iterator&) const = default;
  };
  constexpr iterator begin() { return iterator(cells_); }
  constexpr iterator end() { return iterator(cells_ + size_); }
};

struct Counted {
  int* live;
  int v;
  constexpr Counted(int* l, int x) : live(l), v(x) { ++*live; }
  constexpr Counted(Counted&& o) : live(o.live), v(o.v) { ++*live; }
  constexpr Counted& operator=(Counted&& o) { v = o.v; return *this; }
  constexpr ~Counted() { --*live; }
};

constexpr int constant_test() {
  int live = 0;
  {
    SlotMap<Counted, 4> m;
    Key a = m.emplace(&live, 10);
    Key b = m.emplace(&live, 20);
    Key c = m.emplace(&live, 30);
    if (live != 3 || m.size() != 3) return 1;
    if (!m.erase(a) || m.contains(a) || live != 2) return 2;
    if (m.get(c)->v != 30 || m.get(b)->v != 20) return 3;
    Key d = m.emplace(&live, 40);
    if (d.index != a.index || d.gen == a.gen || m.contains(a)) return 4;
    int sum = 0;
    for (Counted& x : m) sum += x.v;
    if (sum != 90) return 5;
    Key e = m.emplace(&live, 50);
    if (!m.full() || m.emplace(&live, 60) != Key{}) return 6;
    if (!m.erase(e) || !m.erase(b) || m.erase(b)) return 7;
    if (m.size() != 2 || live != 2) return 8;
  }
  return live;
}
static_assert(constant_test() == 0);

struct Particle {
  int x, dx;
};

int main() {
  SlotMap<Particle, 64> world;
  Key keys[64];
  for (int i = 0; i < 64; ++i) keys[i] = world.emplace(Particle{i, i % 3 - 1});
  for (int i = 0; i < 64; i += 2) world.erase(keys[i]);
  if (world.size() != 32) return 1;
  for (int step = 0; step < 10; ++step)
    for (Particle& p : world) p.x += p.dx;
  for (int i = 1; i < 64; i += 2) {
    Particle* p = world.get(keys[i]);
    if (!p || p->x != i + 10 * (i % 3 - 1)) return 2;
  }
  for (int i = 0; i < 64; i += 2)
    if (world.get(keys[i])) return 3;
  Key k = world.emplace(Particle{100, 0});
  if (k.index != 62 || world.get(keys[62])) return 4;
  return 0;
}
