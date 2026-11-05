// scheduler.cpp: a task scheduler with a priority heap, no headers.
// Round 6 sketch, programmer view.
namespace std {
using size_t = decltype(sizeof 0);
}
inline void* operator new(std::size_t, void* p) noexcept { return p; }

enum class Level : unsigned char { low = 1, normal = 5, urgent = 9 };

int rank(Level l) {
  switch (l) {
  case Level::low: return 0;
  case Level::normal: return 1;
  case Level::urgent: return 2;
  }
  return -1;
}

// Every task has a name and an id; Logged and Timed share one Named base.
struct Named {
  const char* name;
  int id;
  explicit Named(const char* n) : name(n), id(next_id()) {}
  static int next_id() {
    static int counter = 0;
    return ++counter;
  }
  virtual ~Named() = default;
};

struct Logged : virtual Named {
  int log_lines = 0;
  Logged() : Named("logged") {}
  void log() { ++log_lines; }
};

struct Timed : virtual Named {
  int ticks = 0;
  Timed() : Named("timed") {}
  void tick(int n) { ticks += n; }
};

struct Failure {
  int code;
};

class Task : public Logged, public Timed {
public:
  Level level;
  int runs = 0;
  Task(const char* n, Level l) : Named(n), level(l) {}
  virtual int run() = 0;
  virtual Task* clone() const = 0;
};

class Counter : public Task {
  int step;
public:
  Counter(const char* n, Level l, int s) : Named(n), Task(n, l), step(s) {}
  int run() override { ++runs; log(); tick(step); return step; }
  Counter* clone() const override { return new Counter(*this); }
};

class Flaky : public Task {
public:
  explicit Flaky(Level l) : Named("flaky"), Task("flaky", l) {}
  int run() override {
    ++runs;
    if (runs % 2 == 1) throw Failure{runs};
    return 100;
  }
  Flaky* clone() const override { return new Flaky(*this); }
};

// A guard that counts how many tasks are in flight.
int in_flight = 0;
struct Flight {
  Flight() { ++in_flight; }
  ~Flight() { --in_flight; }
};

// A binary heap of pointers with a comparison object.
template <class T, class Before, std::size_t N> class Heap {
  alignas(T*) unsigned char raw[N][sizeof(T*)];
  T** slot[N];
  std::size_t used = 0;
  Before before;

  void swap(std::size_t a, std::size_t b) {
    T* t = *slot[a];
    *slot[a] = *slot[b];
    *slot[b] = t;
  }

public:
  Heap() {
    for (std::size_t i = 0; i < N; ++i) slot[i] = ::new (raw[i]) T*(nullptr);
  }
  bool push(T* x) {
    if (used == N) return false;
    std::size_t i = used++;
    *slot[i] = x;
    while (i > 0 && before(*slot[i], *slot[(i - 1) / 2])) {
      swap(i, (i - 1) / 2);
      i = (i - 1) / 2;
    }
    return true;
  }
  T* pop() {
    if (used == 0) return nullptr;
    T* top = *slot[0];
    *slot[0] = *slot[--used];
    std::size_t i = 0;
    for (;;) {
      std::size_t l = 2 * i + 1, r = l + 1, m = i;
      if (l < used && before(*slot[l], *slot[m])) m = l;
      if (r < used && before(*slot[r], *slot[m])) m = r;
      if (m == i) break;
      swap(i, m);
      i = m;
    }
    return top;
  }
  std::size_t size() const { return used; }
};

struct ByLevel {
  bool operator()(const Task* a, const Task* b) const {
    if (rank(a->level) != rank(b->level)) return rank(a->level) > rank(b->level);
    return a->id < b->id;
  }
};

// Runs one task. A Failure is retried once; the retry is the handler's job.
int run_once(Task& t) {
  Flight f;
  try {
    return t.run();
  } catch (const Failure& e) {
    if (e.code > 1) throw;
    return t.run();
  }
}

// A report built in a function-try-block: a bad total turns into code -1.
struct Report {
  int total;
  explicit Report(int t) try : total(check(t)) {
  } catch (const Failure&) {
    throw Failure{-1};
  }
  static int check(int t) {
    if (t < 0) throw Failure{t};
    return t;
  }
};

using Action = void (Timed::*)(int);

int main() {
  int bad = 0;
  Counter a("a", Level::low, 1), b("b", Level::urgent, 2), c("c", Level::normal, 3);
  Flaky f(Level::urgent);
  Heap<Task, ByLevel, 8> heap;
  Task* all[] = {&a, &b, &c, &f};
  for (Task* t : all)
    if (!heap.push(t)) ++bad;

  const char* order[4];
  int total = 0, k = 0;
  while (Task* t = heap.pop()) {
    order[k++] = t->name;
    total += run_once(*t);
  }
  // b and f are urgent; b has the smaller id.
  if (order[0][0] != 'b' || order[1][0] != 'f' || order[2][0] != 'c' || order[3][0] != 'a') ++bad;
  if (total != 2 + 100 + 3 + 1 || in_flight != 0) ++bad;
  if (f.runs != 2 || a.log_lines != 1 || b.ticks != 2) ++bad;

  // The shared base is one object, made by the most derived class (DER-13).
  Logged& lg = c;
  Timed& tm = c;
  if (&static_cast<Named&>(lg) != &static_cast<Named&>(tm) || lg.name[0] != 'c') ++bad;

  Task* copy = c.clone();
  if (copy->run() != 3 || static_cast<Counter*>(copy)->ticks != 6) ++bad;
  delete static_cast<Counter*>(copy);

  Action act = &Timed::tick;
  (a.*act)(10);
  if (a.ticks != 11) ++bad;

  try {
    Report r(-5);
    ++bad;
  } catch (const Failure& e) {
    if (e.code != -1) ++bad;
  }
  Report ok(7);
  if (ok.total != 7) ++bad;

  // A second Flaky run fails with code 3 and is not retried.
  try {
    run_once(f);
    ++bad;
  } catch (const Failure& e) {
    if (e.code != 3 || in_flight != 0) ++bad;
  }
  return bad;
}
