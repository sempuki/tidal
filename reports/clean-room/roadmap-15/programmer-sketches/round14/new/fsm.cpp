// A table-driven state machine. States and events are empty tag types in
// type lists. A transition is a function `on(State, Event)` that returns
// the next state; overload resolution finds it, and a requires expression
// tells whether it exists. The table of next states is built in constant
// evaluation, actions are an overload set of lambdas, and run-time dispatch
// goes through an array of function pointers made by a pack expansion.
// No standard header. Workaround for tidal-fe: the table is a static member
// function in place of a static constexpr data member.

using size_t = decltype(sizeof 0);

namespace fsm {

template <class... Ts> struct List { static constexpr size_t size = sizeof...(Ts); };

template <size_t... I> struct Seq {};

template <size_t N, size_t... I>
struct MakeSeq : MakeSeq<N - 1, N - 1, I...> {};
template <size_t... I>
struct MakeSeq<0, I...> { using type = Seq<I...>; };

template <size_t N> using make_seq = typename MakeSeq<N>::type;

template <class A, class B> inline constexpr bool same = false;
template <class A> inline constexpr bool same<A, A> = true;

// The index of T in the list, or the list's size when T is not there.
template <class T, class... Ts>
constexpr size_t index_of(List<Ts...>) {
  size_t i = 0;
  bool found = false;
  ((found = found || same<T, Ts>, i += found ? 0 : 1), ...);
  return i;
}

template <size_t I, class L> struct At;
template <class T, class... Ts> struct At<0, List<T, Ts...>> { using type = T; };
template <size_t I, class T, class... Ts>
struct At<I, List<T, Ts...>> : At<I - 1, List<Ts...>> {};

template <class... F>
struct overloaded : F... {
  using F::operator()...;
};

inline constexpr size_t none = static_cast<size_t>(-1);

template <class S, class E>
concept has_transition = requires(S s, E e) { on(s, e); };

template <class States, class Events>
struct Machine;

template <class... S, class... E>
struct Machine<List<S...>, List<E...>> {
  using States = List<S...>;
  using Events = List<E...>;

  template <class St, class Ev>
  static constexpr size_t next_of() {
    if constexpr (has_transition<St, Ev>)
      return index_of<decltype(on(St{}, Ev{}))>(States{});
    else
      return none;
  }

  // table[s][e] is the index of the next state, or none.
  struct Table {
    size_t next[sizeof...(S)][sizeof...(E)];
  };

  template <class St>
  static constexpr void fill_row(size_t (&row)[sizeof...(E)]) {
    size_t e = 0;
    ((row[e++] = next_of<St, E>()), ...);
  }

  static constexpr Table make_table() {
    Table t{};
    size_t s = 0;
    (fill_row<S>(t.next[s++]), ...);
    return t;
  }

  // Workaround: a member function, not a static data member, so that the
  // table is made after the class is complete.
  static constexpr Table table() { return make_table(); }

  size_t state = 0;
  int rejected = 0;

  template <class Ev>
  constexpr bool fire(Ev) {
    constexpr size_t e = index_of<Ev>(Events{});
    static_assert(e < sizeof...(E), "not an event of this machine");
    size_t n = table().next[state][e];
    if (n == none) {
      ++rejected;
      return false;
    }
    state = n;
    return true;
  }

  template <class St>
  constexpr bool in() const { return state == index_of<St>(States{}); }

  // Calls the action for the current state with the state's tag.
  template <class Actions, size_t... I>
  static constexpr int dispatch_impl(const Actions& a, size_t s, Seq<I...>) {
    using Fn = int (*)(const Actions&);
    constexpr Fn fns[] = {+[](const Actions& x) -> int {
      return x(typename At<I, States>::type{});
    }...};
    return fns[s](a);
  }

  template <class Actions>
  constexpr int act(const Actions& a) const {
    return dispatch_impl(a, state, make_seq<sizeof...(S)>{});
  }
};

}  // namespace fsm

namespace door {

struct Closed {};
struct Open {};
struct Locked {};

struct Push {};
struct Pull {};
struct Lock {};
struct Unlock {};

constexpr Open on(Closed, Pull) { return {}; }
constexpr Closed on(Open, Push) { return {}; }
constexpr Locked on(Closed, Lock) { return {}; }
constexpr Closed on(Locked, Unlock) { return {}; }

using Door = fsm::Machine<fsm::List<Closed, Open, Locked>, fsm::List<Push, Pull, Lock, Unlock>>;

}  // namespace door

static_assert(fsm::has_transition<door::Closed, door::Pull>);
static_assert(!fsm::has_transition<door::Open, door::Lock>);
static_assert(fsm::index_of<door::Locked>(fsm::List<door::Closed, door::Open, door::Locked>{}) == 2);
static_assert(fsm::index_of<int>(fsm::List<door::Closed, door::Open>{}) == 2);
static_assert(fsm::same<fsm::At<1, fsm::List<int, char, long>>::type, char>);
static_assert(door::Door::table().next[0][1] == 1);
static_assert(door::Door::table().next[1][2] == fsm::none);
static_assert(door::Door::table().next[2][3] == 0);

constexpr auto describe = fsm::overloaded{
    [](door::Closed) { return 10; },
    [](door::Open) { return 20; },
    [](door::Locked) { return 30; },
};

constexpr int scenario() {
  door::Door d;
  int trace = 0;
  d.fire(door::Pull{});    // Closed -> Open
  trace = trace * 100 + d.act(describe);
  d.fire(door::Lock{});    // rejected
  d.fire(door::Push{});    // Open -> Closed
  d.fire(door::Lock{});    // Closed -> Locked
  trace = trace * 100 + d.act(describe);
  d.fire(door::Pull{});    // rejected
  d.fire(door::Unlock{});  // Locked -> Closed
  trace = trace * 100 + d.act(describe);
  return d.in<door::Closed>() && d.rejected == 2 ? trace : -1;
}

static_assert(scenario() == 203010);

int main() {
  int r = 0;
  if (scenario() != 203010) r |= 1;
  door::Door d;
  int k = 0;
  auto count = fsm::overloaded{
      [&](door::Closed) { return ++k; },
      [&](auto) { return 0; },
  };
  d.act(count);
  if (k != 1) r |= 2;
  return r;
}
