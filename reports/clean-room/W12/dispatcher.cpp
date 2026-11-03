// W12: an event dispatcher. Handlers of any callable type are erased
// behind a virtual base, so one dispatcher holds closures, function
// pointers, and bound member functions (CE-1 permits virtual calls and
// new/delete in constant evaluation; CE-4 rules out a void* scheme).

enum class Event { Click, Key, Count };

struct Handler {
  constexpr virtual void call(int value) = 0;
  constexpr virtual ~Handler() = default;
};

template <class F>
struct Holder final : Handler {
  F f;
  constexpr explicit Holder(F fn) : f(fn) {}
  constexpr void call(int value) override { f(value); }
};

class Dispatcher {
  static constexpr int max_handlers = 8;
  static constexpr int events = static_cast<int>(Event::Count);
  Handler* slots[events][max_handlers] = {};

public:
  constexpr Dispatcher() = default;
  Dispatcher(const Dispatcher&) = delete;
  Dispatcher& operator=(const Dispatcher&) = delete;
  constexpr ~Dispatcher() {
    for (auto& row : slots)
      for (Handler* h : row) delete h;
  }

  // Returns an id, or -1 when the event has no free slot.
  template <class F>
  constexpr int subscribe(Event e, F f) {
    int row = static_cast<int>(e);
    for (int i = 0; i < max_handlers; ++i) {
      if (slots[row][i] == nullptr) {
        slots[row][i] = new Holder<F>(f);
        return row * max_handlers + i;
      }
    }
    return -1;
  }

  // Binds an object and a pointer to member function (MPTR-3).
  template <class C>
  constexpr int subscribe(Event e, C& obj, void (C::*m)(int)) {
    return subscribe(e, [&obj, m](int v) { (obj.*m)(v); });
  }

  constexpr bool unsubscribe(int id) {
    if (id < 0) return false;
    Handler*& h = slots[id / max_handlers][id % max_handlers];
    if (h == nullptr) return false;
    delete h;
    h = nullptr;
    return true;
  }

  constexpr int emit(Event e, int value) {
    int called = 0;
    for (Handler* h : slots[static_cast<int>(e)]) {
      if (h != nullptr) {
        h->call(value);
        ++called;
      }
    }
    return called;
  }
};

struct Score {
  int total = 0;
  constexpr void add(int v) { total += v; }
  constexpr void sub(int v) { total -= v; }
};

constexpr void ignore(int) {}

constexpr int scenario() {
  int clicks = 0;
  int keys = 0;
  int last_key = 0;
  Score score;
  Dispatcher d;
  d.subscribe(Event::Click, [&clicks](int) { ++clicks; });
  int key_id = d.subscribe(Event::Key, [&](int k) { ++keys; last_key = k; });
  d.subscribe(Event::Click, score, &Score::add);
  d.subscribe(Event::Key, score, &Score::sub);
  d.subscribe(Event::Key, ignore);                   // a function pointer
  int counted = d.emit(Event::Click, 10);            // 2 handlers
  counted += d.emit(Event::Click, 5);                // 2 handlers
  counted += d.emit(Event::Key, 3);                  // 3 handlers
  bool removed = d.unsubscribe(key_id);
  bool removed_twice = d.unsubscribe(key_id);
  counted += d.emit(Event::Key, 4);                  // 2 handlers
  // clicks 2, keys 1, last_key 3, score 10 + 5 - 3 - 4 = 8, counted 9
  return clicks == 2 && keys == 1 && last_key == 3 && score.total == 8 &&
         counted == 9 && removed && !removed_twice;
}
static_assert(scenario());

int main() { return scenario() ? 0 : 1; }
