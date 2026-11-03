// W10: a cleanup guard in place of Go's defer.
// The guard holds a closure and calls it in its destructor. The
// destructor runs when control leaves the guard's block by any route,
// a throw included (OBJ-P2, UNW-1). Guards end last-made-first, as Go
// runs defers. Go runs defers at function end; a guard ends at the end
// of its own block.

template <class F>
class Defer {
  F fn;
  bool active = true;
public:
  constexpr explicit Defer(F f) : fn(f) {}
  Defer(const Defer&) = delete;             // CPY-P2: one owner
  Defer& operator=(const Defer&) = delete;
  constexpr void cancel() noexcept { active = false; }
  constexpr ~Defer() {                       // noexcept by DTOR-4
    if (active) fn();
  }
};

// A prvalue initializes the target directly, so a class with no copy
// or move can still be returned (CPY-P4).
template <class F>
constexpr Defer<F> make_defer(F f) { return Defer<F>(f); }

struct Log {
  int items[16];
  int n = 0;
  constexpr void add(int v) { items[n++] = v; }
};

constexpr bool order_check() {
  Log log;
  {
    Defer a([&] { log.add(1); });           // CTAD from the constructor
    Defer b([&] { log.add(2); });
    auto c = make_defer([&] { log.add(3); });
    Defer d([&] { log.add(4); });
    d.cancel();                              // like a dismissed defer
    log.add(0);
  }
  // body first, then 3, 2, 1
  return log.n == 4 && log.items[0] == 0 && log.items[1] == 3 &&
         log.items[2] == 2 && log.items[3] == 1;
}
static_assert(order_check());

constexpr int loop_check() {
  Log log;
  for (int i = 0; i < 3; ++i) {
    Defer g([&] { log.add(10 + i); });       // runs at each iteration end
    log.add(i);
  }
  return log.n == 6 && log.items[0] == 0 && log.items[1] == 10 &&
         log.items[4] == 2 && log.items[5] == 12;
}
static_assert(loop_check());

// The guard's destructor is noexcept, so a cleanup that throws would
// end the program (UNW-5).
using Probe = Defer<void (*)()>;
static_assert(noexcept(static_cast<Probe*>(nullptr)->~Probe()));

struct Failure { int code; };

int resources_open = 0;

void open_resource() { ++resources_open; }
void close_resource() noexcept { --resources_open; }

int work(bool fail) {
  open_resource();
  Defer close([] { close_resource(); });
  open_resource();
  Defer close2([] { close_resource(); });
  if (fail) throw Failure{7};               // both guards run first
  return resources_open;
}

int main() {
  if (work(false) != 2) return 1;
  if (resources_open != 0) return 2;
  try {
    work(true);
    return 3;
  } catch (const Failure& f) {
    if (f.code != 7) return 4;
    if (resources_open != 0) return 5;      // UNW-1: done before the handler
  }

  // An early return also runs the guard.
  int calls = 0;
  auto early = [&](bool stop) {
    Defer g([&] { ++calls; });
    if (stop) return 1;
    return 2;
  };
  if (early(true) != 1 || early(false) != 2 || calls != 2) return 6;
  return 0;
}
