// tidal-rules: VIRT-1 VIRT-3 VIRT-7 VIRT-8 VIRT-10 DER-5 CE-1 CE-2 CE-7 LSUP-8 TRY-2
// tidal-expect: run
// tidal-note: Write suite W9, program plugin.cpp (reports/clean-room/W9/report.md). The rules are the ones in the report's construct-to-rule notes that the program's comments name.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// W9: a plugin interface with two implementations.
// Rules used: VIRT-1, VIRT-3, VIRT-7, VIRT-8, VIRT-10, VIRT-11, DER-5,
// CE-1, CE-2, CE-7, LSUP-8, TRY-2.

namespace std {
class exception {
public:
  exception() noexcept {}
  virtual ~exception();             // the library defines it (LSUP-8)
};
class bad_cast : public exception {
public:
  bad_cast() noexcept {}
  ~bad_cast() override;
};
}

// The interface. Pure virtual functions make Plugin abstract (VIRT-8).
// The virtual destructor lets delete through Plugin* reach the whole
// object (VIRT-7).
struct Plugin {
  virtual ~Plugin() = default;
  constexpr virtual int id() const = 0;
  constexpr virtual int run(int x) const = 0;
  // A non-virtual helper that makes virtual calls (VIRT-1).
  constexpr int twice(int x) const { return run(run(x)); }
};

struct Doubler final : Plugin {
  constexpr int id() const override { return 1; }
  constexpr int run(int x) const override { return 2 * x; }
};

struct Adder : Plugin {
  int k;
  constexpr explicit Adder(int step) : k(step) {}
  constexpr int id() const override { return 2; }
  constexpr int run(int x) const override { return x + k; }
};

// The registry hands out plugins through the base (DER-5).
constexpr Plugin* make(int which) {
  if (which == 1) return new Doubler;
  if (which == 2) return new Adder(3);
  return nullptr;
}

constexpr int use(const Plugin& p, int x) { return p.twice(x); }

constexpr int on_stack() {
  Doubler d;
  Adder a{3};
  const Plugin* ps[] = {&d, &a};
  int sum = 0;
  for (const Plugin* p : ps) sum += use(*p, 1);       // 4 + 7
  if (dynamic_cast<const Adder*>(ps[0]) != nullptr) return -1;   // VIRT-10
  if (dynamic_cast<const Adder*>(ps[1])->k != 3) return -2;
  if (dynamic_cast<const Doubler*>(ps[0]) != &d) return -3;
  return sum;
}

constexpr int on_heap() {
  int sum = 0;
  for (int w = 1; w <= 2; ++w) {
    Plugin* p = make(w);
    sum += p->id() * 100 + p->run(10);
    delete p;                       // CE-7: freed in the same evaluation
  }
  return sum;                       // (100 + 20) + (200 + 13)
}

static_assert(on_stack() == 11);
static_assert(on_heap() == 333);

int main() {
  if (on_stack() != 11) return 1;
  if (on_heap() != 333) return 2;

  Plugin* p = make(1);
  if (p == nullptr || p->run(21) != 42) return 3;

  // A failed reference cast throws std::bad_cast (VIRT-10, LSUP-8).
  int caught = 0;
  try {
    const Adder& a = dynamic_cast<const Adder&>(*p);
    (void)a;
  } catch (const std::bad_cast&) {
    caught = 1;
  }
  delete p;
  if (caught != 1) return 4;
  if (make(9) != nullptr) return 5;
  return 0;
}
