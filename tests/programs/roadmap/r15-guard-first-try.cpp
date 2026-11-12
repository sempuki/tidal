// tidal-rules: TRY-10 THR-6 THR-8 UNW-1 EXS-1
// tidal-expect: run
// tidal-note: Roadmap round 15, programmer sketch guard_first_try.cpp (reports/clean-room/roadmap-15/programmer.md, section 1.4). The first try builds clean on GCC and Clang, so it is a program test (decision 0061). It differs from the last version in a global log named log in place of journal, and in no test of a rethrow and of a handler that slices. The report names no rule for this program. The rules are those of the constructs that the report's table names for it: conditional noexcept, a function try block on a constructor, a rethrow, and a handler that slices, with the destructors that run as an exception leaves a scope.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// Scope guards and owned resources with exceptions: ScopeExit runs a
// callable when its scope ends, Rollback runs one unless the work
// commits, and Resource owns a handle with a deleter. A small key-value
// store applies a batch of changes as one transaction, and undoes the
// applied ones when a later change throws. No standard header.

namespace sg {

template <class T> struct remove_ref { using type = T; };
template <class T> struct remove_ref<T&> { using type = T; };
template <class T> struct remove_ref<T&&> { using type = T; };

template <class T>
constexpr typename remove_ref<T>::type&& move(T&& t) noexcept {
  return static_cast<typename remove_ref<T>::type&&>(t);
}

template <class T>
T&& declval() noexcept;

template <class F>
class ScopeExit {
 public:
  explicit ScopeExit(F f) noexcept(noexcept(F(move(f)))) : f_(move(f)) {}
  ScopeExit(ScopeExit&& other) noexcept(noexcept(F(move(other.f_))))
      : f_(move(other.f_)), active_(other.active_) {
    other.release();
  }
  ScopeExit(const ScopeExit&) = delete;
  ScopeExit& operator=(const ScopeExit&) = delete;
  ~ScopeExit() noexcept {
    if (active_) f_();
  }
  void release() noexcept { active_ = false; }

 private:
  F f_;
  bool active_ = true;
};

template <class F>
class Rollback {
 public:
  explicit Rollback(F f) : f_(move(f)) {}
  Rollback(const Rollback&) = delete;
  ~Rollback() noexcept {
    if (!committed_) f_();
  }
  void commit() noexcept { committed_ = true; }

 private:
  F f_;
  bool committed_ = false;
};

template <class H, class D>
class Resource {
 public:
  Resource(H h, D d) noexcept : h_(h), d_(move(d)), owns_(true) {}
  Resource(Resource&& o) noexcept : h_(o.h_), d_(move(o.d_)), owns_(o.owns_) { o.owns_ = false; }
  Resource& operator=(Resource&& o) noexcept {
    if (this != &o) {
      reset();
      h_ = o.h_;
      d_ = move(o.d_);
      owns_ = o.owns_;
      o.owns_ = false;
    }
    return *this;
  }
  ~Resource() { reset(); }
  void reset() noexcept {
    if (owns_) d_(h_);
    owns_ = false;
  }
  const H& get() const noexcept { return h_; }

 private:
  H h_;
  D d_;
  bool owns_;
};

}  // namespace sg

// A log of events in a fixed buffer.
struct Log {
  char text[256]{};
  int len = 0;
  void put(const char* s) {
    while (*s && len < 255) text[len++] = *s++;
  }
  void put(char c) {
    if (len < 255) text[len++] = c;
  }
  bool is(const char* s) const {
    int i = 0;
    for (; s[i]; ++i)
      if (i >= len || text[i] != s[i]) return false;
    return i == len;
  }
};

Log log;

struct Error {
  virtual ~Error() = default;
  virtual const char* what() const noexcept { return "error"; }
};
struct Full : Error {
  const char* what() const noexcept override { return "full"; }
};
struct BadKey : Error {
  char key;
  explicit BadKey(char k) : key(k) {}
  const char* what() const noexcept override { return "bad key"; }
};

class Store {
 public:
  static constexpr int cap = 4;

  void put(char key, int value) {
    if (key < 'a' || key > 'z') throw BadKey(key);
    for (int i = 0; i < n_; ++i) {
      if (keys_[i] == key) {
        vals_[i] = value;
        return;
      }
    }
    if (n_ == cap) throw Full{};
    keys_[n_] = key;
    vals_[n_] = value;
    ++n_;
  }
  bool get(char key, int& out) const {
    for (int i = 0; i < n_; ++i)
      if (keys_[i] == key) {
        out = vals_[i];
        return true;
      }
    return false;
  }
  void remove(char key) {
    for (int i = 0; i < n_; ++i)
      if (keys_[i] == key) {
        keys_[i] = keys_[n_ - 1];
        vals_[i] = vals_[n_ - 1];
        --n_;
        return;
      }
  }
  int size() const { return n_; }

 private:
  char keys_[cap]{};
  int vals_[cap]{};
  int n_ = 0;
};

struct Change {
  char key;
  int value;
};

// Applies every change or none. Each applied change pushes an undo step
// on a fixed stack, and a guard runs the stack backward unless the
// batch commits.
void apply(Store& s, const Change* changes, int n) {
  struct Undo {
    char key;
    bool had;
    int old;
  };
  Undo undo[8];
  int top = 0;
  sg::Rollback guard([&] {
    while (top > 0) {
      const Undo& u = undo[--top];
      if (u.had) s.put(u.key, u.old);
      else s.remove(u.key);
      log.put('u');
    }
  });
  for (int i = 0; i < n; ++i) {
    int old = 0;
    bool had = s.get(changes[i].key, old);
    s.put(changes[i].key, changes[i].value);
    undo[top++] = {changes[i].key, had, old};
  }
  guard.commit();
}

// A member that throws in a constructor: the members built before it
// are destroyed, and the function try block sees the exception.
struct Part {
  char name;
  explicit Part(char c, bool fail) : name(c) {
    if (fail) throw Error{};
    log.put(name);
  }
  ~Part() { log.put('~'), log.put(name); }
};
struct Machine {
  Part a, b, c;
  Machine(bool fail_b) try : a('A', false), b('B', fail_b), c('C', false) {
    log.put('+');
  } catch (const Error&) {
    log.put('!');
  }
};

int closes = 0;

int run() {
  {
    sg::ScopeExit e([] { log.put('1'); });
    sg::ScopeExit f([] { log.put('2'); });
    auto g = sg::ScopeExit([] { log.put('3'); });
    g.release();
  }
  if (!log.is("21")) return 1;

  Store s;
  Change ok[] = {{'a', 1}, {'b', 2}};
  apply(s, ok, 2);
  int v = 0;
  if (s.size() != 2 || !s.get('b', v) || v != 2) return 2;

  Change bad[] = {{'a', 10}, {'c', 3}, {'d', 4}, {'e', 5}};
  try {
    apply(s, bad, 4);
    return 3;
  } catch (const Error& e) {
    log.put(e.what());
  }
  if (!log.is("21uuufull")) return 4;
  if (s.size() != 2 || !s.get('a', v) || v != 1 || s.get('c', v)) return 5;

  try {
    Change worse[] = {{'b', 7}, {'?', 0}};
    apply(s, worse, 2);
  } catch (const BadKey& e) {
    if (e.key != '?') return 6;
  }
  if (!s.get('b', v) || v != 2) return 7;

  log = Log{};
  try {
    Machine m(true);
    return 8;
  } catch (const Error&) {
    log.put('$');
  }
  if (!log.is("A~A!$")) return 9;

  {
    auto closer = [](int h) { closes += h; };
    sg::Resource r(3, closer);
    sg::Resource q = sg::move(r);
    sg::Resource t(4, closer);
    t = sg::move(q);
    if (closes != 4 || t.get() != 3) return 10;
  }
  if (closes != 7) return 11;

  static_assert(noexcept(sg::ScopeExit([]() noexcept {})));
  return 0;
}

int main() {
  try {
    return run();
  } catch (...) {
    return 100;
  }
}
