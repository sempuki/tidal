// W10: noexcept functions that must not throw.
// Go has no checked "cannot fail" marker. Here noexcept is a promise
// in the function type. Breaking it at run time ends the program
// (THR-9, UNW-5), so a noexcept function catches inside itself (UNW-6).

namespace std {
class exception {
public:
  exception() noexcept {}
  virtual ~exception();
};
}

struct Overflow : std::exception {};

// 1. The specifier and the operator.
constexpr int add(int a, int b) noexcept { return a + b; }
constexpr int sq(int x) { return x * x; }       // constexpr does not imply noexcept
void may_fail();                                  // never called

static_assert(noexcept(add(1, 2)));
static_assert(!noexcept(sq(2)));                  // EXS-4
static_assert(!noexcept(may_fail()));             // UN-18
static_assert(!noexcept([] { return 1; }()));     // lambda: only with noexcept
static_assert(noexcept([]() noexcept { return 1; }()));

// 2. The specifier travels with the type (EXS-2, CONV-10).
using SafeOp = int (*)(int, int) noexcept;
using AnyOp = int (*)(int, int);
constexpr SafeOp safe = add;
constexpr AnyOp any = add;                        // noexcept dropped: fine
static_assert(noexcept(safe(1, 2)));
static_assert(!noexcept(any(1, 2)));

constexpr int pick(void (*)() noexcept) { return 1; }
constexpr int pick(void (*)()) { return 2; }
void quiet() noexcept {}
void loud() {}
static_assert(pick(quiet) == 1 && pick(loud) == 2);   // ICS-26

// 3. A conditional promise in a template (EXS-9).
struct Cheap { int v; };
struct Costly {
  int v;
  Costly(int x) : v(x) {}
  Costly(const Costly& o) : v(o.v) {}             // may throw
  Costly& operator=(const Costly& o) { v = o.v; return *this; }
};

template <class T>
constexpr void exchange(T& a, T& b) noexcept(noexcept(T(a)) && noexcept(a = b)) {
  T t(a);
  a = b;
  b = t;
}
static_assert(noexcept(exchange(*static_cast<Cheap*>(nullptr),
                                *static_cast<Cheap*>(nullptr))));
static_assert(!noexcept(exchange(*static_cast<Costly*>(nullptr),
                                 *static_cast<Costly*>(nullptr))));

// 4. Implicit members follow their parts (EXS-4, DTOR-4).
struct Holder { Cheap c; };
struct HolderCostly { Costly c; };
static_assert(noexcept(Holder(Holder{})));
static_assert(!noexcept(HolderCostly(*static_cast<HolderCostly*>(nullptr))));
struct Calm { ~Calm() {} };                        // no specifier: noexcept
static_assert(noexcept(static_cast<Calm*>(nullptr)->~Calm()));
struct Loud { ~Loud() noexcept(false) {} };
static_assert(!noexcept(static_cast<Loud*>(nullptr)->~Loud()));

// 5. An override keeps the base's promise (EXS-3).
struct Sink {
  virtual int put(int v) noexcept { return v; }
  virtual ~Sink() = default;
};
struct Doubler : Sink {
  int put(int v) noexcept override { return 2 * v; }
};

// 6. A noexcept wrapper around code that throws: catch inside.
constexpr int checked_mul(int a, int b) {
  if (b != 0 && (a > 46340 || b > 46340)) throw Overflow{};
  return a * b;
}
constexpr int saturating_mul(int a, int b) noexcept {
  try {
    return checked_mul(a, b);
  } catch (const Overflow&) {
    return 2147483647;
  }
}
static_assert(saturating_mul(3, 4) == 12);        // no throw: CE-1

// 7. A destructor marked noexcept(false) may throw when nothing else is
// in flight (UNW-6).
int destroyed = 0;
struct Fussy {
  bool fail;
  ~Fussy() noexcept(false) {
    ++destroyed;
    if (fail) throw 3;
  }
};

int main() {
  if (safe(2, 3) != 5 || any(2, 3) != 5) return 1;

  Cheap a{1}, b{2};
  exchange(a, b);
  if (a.v != 2 || b.v != 1) return 2;
  Costly c(1), d(2);
  exchange(c, d);
  if (c.v != 2 || d.v != 1) return 3;

  Doubler dbl;
  Sink& s = dbl;
  if (s.put(4) != 8) return 4;

  if (saturating_mul(50000, 50000) != 2147483647) return 5;
  if (saturating_mul(6, 7) != 42) return 6;

  try {
    Fussy f{true};
  } catch (int v) {
    if (v != 3 || destroyed != 1) return 7;
  }
  if (destroyed != 1) return 8;
  return 0;
}
