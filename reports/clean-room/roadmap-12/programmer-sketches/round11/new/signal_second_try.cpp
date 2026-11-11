// Signal<R(Args...)>: a partial specialization on a function type, slots in
// an intrusive doubly linked list, connect for a free function, for a
// member function given as an auto template argument, and for any
// callable object kept by reference; Connection is a move-only owner
// that disconnects in its destructor through a virtual base; emit is safe
// when a slot disconnects itself or another slot during the emission; a
// collect form folds the results.
namespace std { using size_t = decltype(sizeof 0); }

template <class T> struct remove_ref { using type = T; };
template <class T> struct remove_ref<T&> { using type = T; };
template <class T> struct remove_ref<T&&> { using type = T; };
template <class T> constexpr typename remove_ref<T>::type&& move(T&& t) noexcept {
  return static_cast<typename remove_ref<T>::type&&>(t);
}

template <class M> struct member_traits;
template <class R, class C, class... A> struct member_traits<R (C::*)(A...)> { using cls = C; };
template <class R, class C, class... A> struct member_traits<R (C::*)(A...) const> { using cls = const C; };

struct SlotBase {
  SlotBase* prev = nullptr;
  SlotBase* next = nullptr;
  bool dead = false;
  virtual ~SlotBase() = default;
};

class SignalBase {
public:
  virtual void drop(SlotBase* s) noexcept = 0;
protected:
  ~SignalBase() = default;
};

class Connection {
  SignalBase* sig_ = nullptr;
  SlotBase* slot_ = nullptr;
public:
  Connection() = default;
  Connection(SignalBase* s, SlotBase* sl) : sig_(s), slot_(sl) {}
  Connection(const Connection&) = delete;
  Connection& operator=(const Connection&) = delete;
  Connection(Connection&& o) noexcept : sig_(o.sig_), slot_(o.slot_) { o.sig_ = nullptr; o.slot_ = nullptr; }
  Connection& operator=(Connection&& o) noexcept {
    if (this != &o) { disconnect(); sig_ = o.sig_; slot_ = o.slot_; o.sig_ = nullptr; o.slot_ = nullptr; }
    return *this;
  }
  ~Connection() { disconnect(); }
  bool connected() const { return sig_ != nullptr; }
  void disconnect() noexcept {
    if (sig_) sig_->drop(slot_);
    sig_ = nullptr;
    slot_ = nullptr;
  }
  SlotBase* release() noexcept { SlotBase* s = slot_; sig_ = nullptr; slot_ = nullptr; return s; }
};

template <class Sig> class Signal;

template <class R, class... Args> class Signal<R(Args...)> final : public SignalBase {
  struct Slot : SlotBase {
    void* obj;
    R (*call)(void*, Args...);
    Slot(void* o, R (*c)(void*, Args...)) : obj(o), call(c) {}
  };
  SlotBase head_;
  int depth_ = 0;
  int pending_ = 0;

  Connection add(void* obj, R (*call)(void*, Args...)) {
    Slot* s = new Slot(obj, call);
    s->prev = head_.prev;
    s->next = &head_;
    head_.prev->next = s;
    head_.prev = s;
    return Connection(this, s);
  }
  static void unlink(SlotBase* s) noexcept {
    s->prev->next = s->next;
    s->next->prev = s->prev;
    delete s;
  }
  void sweep() noexcept {
    SlotBase* s = head_.next;
    while (s != &head_) {
      SlotBase* n = s->next;
      if (s->dead) unlink(s);
      s = n;
    }
    pending_ = 0;
  }

public:
  Signal() { head_.prev = head_.next = &head_; }
  Signal(const Signal&) = delete;
  ~Signal() {
    while (head_.next != &head_) unlink(head_.next);
  }
  void drop(SlotBase* s) noexcept override {
    if (depth_) { s->dead = true; ++pending_; } else unlink(s);
  }
  std::size_t size() const {
    std::size_t n = 0;
    for (const SlotBase* s = head_.next; s != &head_; s = s->next) if (!s->dead) ++n;
    return n;
  }

  Connection connect(R (*fn)(Args...)) {
    return add(reinterpret_cast<void*>(fn), [](void* f, Args... a) -> R {
      return reinterpret_cast<R (*)(Args...)>(f)(a...);
    });
  }
  template <auto Mem> Connection connect(typename member_traits<decltype(Mem)>::cls& obj) {
    using C = typename member_traits<decltype(Mem)>::cls;
    return add(const_cast<void*>(static_cast<const void*>(&obj)), [](void* o, Args... a) -> R {
      return (static_cast<C*>(o)->*Mem)(a...);
    });
  }
  template <class F> Connection connect(F& f) {
    return add(&f, [](void* o, Args... a) -> R { return (*static_cast<F*>(o))(a...); });
  }

  void emit(Args... args) {
    ++depth_;
    for (SlotBase* s = head_.next; s != &head_; s = s->next)
      if (!s->dead) static_cast<Slot*>(s)->call(static_cast<Slot*>(s)->obj, args...);
    if (--depth_ == 0 && pending_) sweep();
  }
  template <class T, class Op> T collect(T init, Op op, Args... args) {
    ++depth_;
    for (SlotBase* s = head_.next; s != &head_; s = s->next)
      if (!s->dead) init = op(move(init), static_cast<Slot*>(s)->call(static_cast<Slot*>(s)->obj, args...));
    if (--depth_ == 0 && pending_) sweep();
    return init;
  }
};

inline int total = 0;
void add_total(int v) { total += v; }
int twice(int v) { return 2 * v; }

struct Counter {
  int hits = 0;
  int sum = 0;
  void on(int v) { ++hits; sum += v; }
  int scaled(int v) const { return v * 10; }
};

int main() {
  int fails = 0;
  {
    Signal<void(int)> changed;
    Counter c;
    Connection a = changed.connect(add_total);
    // Second try: a lambda, since the front end stops on
    // connect<&Counter::on>(c) (a pointer to member as a template argument).
    auto on = [&c](int v) { c.on(v); };
    Connection b = changed.connect(on);
    int seen = 0;
    auto watcher = [&seen](int v) { seen = v; };
    Connection w = changed.connect(watcher);
    changed.emit(3);
    if (total != 3 || c.hits != 1 || c.sum != 3 || seen != 3) ++fails;
    b.disconnect();
    changed.emit(4);
    if (total != 7 || c.hits != 1 || seen != 4 || changed.size() != 2) ++fails;
    {
      Connection moved = move(w);
      if (w.connected() || !moved.connected()) ++fails;
    }
    if (changed.size() != 1) ++fails;

    Connection self;
    auto once = [&self, &c](int v) { c.sum += v; self.disconnect(); };
    self = changed.connect(once);
    Connection later;
    auto killer = [&later](int) { later.disconnect(); };
    Connection k = changed.connect(killer);
    auto never = [&fails](int) { ++fails; };
    later = changed.connect(never);
    changed.emit(5);
    if (c.sum != 8 || total != 12 || changed.size() != 2 || self.connected()) ++fails;
  }
  {
    Signal<int(int)> query;
    const Counter c;
    Connection p = query.connect(twice);
    auto scaled = [&c](int v) { return c.scaled(v); };
    Connection q = query.connect(scaled);
    int sum = query.collect(0, [](int acc, int r) { return acc + r; }, 4);
    if (sum != 8 + 40) ++fails;
    SlotBase* leaked = p.release();
    if (query.size() != 2 || leaked == nullptr) ++fails;
  }
  return fails;
}
