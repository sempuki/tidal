// tidal-rules: CPY-1 CPY-2 CPY-3 CPY-4 CPY-5 CPY-21 CPY-9 CPY-10 CPY-11 CPY-12 NEW-1 NEW-4 NEW-5 NEW-9 DTOR-5
// tidal-expect: run
// tidal-note: Write suite W8, program unique.cpp (reports/clean-room/W8/report.md). The rules are the ones the report's notes made before writing give for this program.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// A unique owner of one object, written from the Tidal specification.
// Rules used: CPY-P2, CPY-2, CPY-3, CPY-4, CPY-5, CPY-9, CPY-11, CPY-21,
// NEW-1, NEW-4, NEW-5, NEW-9, DTOR-5, OPR (operator->), CVF (explicit bool).

int live = 0;     // number of Widget objects that exist now
int destroyed = 0;

struct Widget {
  int value;
  explicit Widget(int v) : value(v) { ++live; }
  ~Widget() { --live; ++destroyed; }
};

template <class T>
class Unique {
  T* ptr = nullptr;

public:
  Unique() = default;
  explicit Unique(T* p) : ptr(p) {}

  // A copy would share the object, and both owners would delete it (NEW-5).
  Unique(const Unique&) = delete;
  Unique& operator=(const Unique&) = delete;

  // The move takes the pointer and leaves the source empty.
  Unique(Unique&& other) noexcept : ptr(other.ptr) { other.ptr = nullptr; }
  Unique& operator=(Unique&& other) noexcept {
    if (this != &other) {
      delete ptr;               // delete of null does nothing (NEW-4)
      ptr = other.ptr;
      other.ptr = nullptr;
    }
    return *this;
  }

  ~Unique() { delete ptr; }

  T* get() const { return ptr; }
  T& operator*() const { return *ptr; }
  T* operator->() const { return ptr; }
  explicit operator bool() const { return ptr != nullptr; }

  T* release() {
    T* p = ptr;
    ptr = nullptr;
    return p;
  }
  void reset(T* p = nullptr) {
    T* old = ptr;
    ptr = p;
    delete old;
  }
};

template <class T, class... Args>
Unique<T> make_unique(Args&&... args) {
  return Unique<T>(new T(static_cast<Args&&>(args)...));  // prvalue: no move (CPY-10)
}

Unique<Widget> build(int v) {
  Unique<Widget> local = make_unique<Widget>(v);
  local->value += 1;
  return local;                   // moved, or is the return value (CPY-11, CPY-12)
}

int main() {
  {
    Unique<Widget> a = build(41);
    if (!a) return 1;
    if (a->value != 42) return 2;
    if (live != 1) return 3;

    Unique<Widget> b = static_cast<Unique<Widget>&&>(a);  // move (CPY-9)
    if (a) return 4;
    if ((*b).value != 42) return 5;
    if (live != 1) return 6;

    Unique<Widget> c(new Widget(7));
    c = static_cast<Unique<Widget>&&>(b);    // c's old Widget is deleted
    if (live != 1) return 7;
    if (c->value != 42) return 8;
    if (b.get() != nullptr) return 9;

    Unique<Widget>& same = c;
    c = static_cast<Unique<Widget>&&>(same); // self-move keeps the object
    if (!c || c->value != 42) return 10;

    Widget* raw = c.release();
    if (c) return 11;
    if (live != 1) return 12;
    c.reset(raw);
    c.reset(new Widget(3));                   // the 42 Widget is deleted
    if (live != 1 || c->value != 3) return 13;
    c.reset();
    if (live != 0) return 14;
  }
  if (live != 0) return 15;
  if (destroyed != 3) return 16;   // three Widgets were made
  return 0;
}
