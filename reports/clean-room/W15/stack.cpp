// W15 exercise 1: a fixed-capacity stack whose capacity is a template
// parameter. Written from the Tidal specification by a clean-room reader.
//
// Rules used:
//   TPL-1, TPL-6     class template with a type and a constant parameter
//   CPT-8, CPT-10    requires clause after the head; (Capacity > 0) needs
//                    parentheses
//   CNS-13, CPT-32   Stack<int, 0> fails its constraint; tested through a
//                    concept so that CPT-27 does not reject the program
//   UNI-1, UNI-3     a union slot with a user-provided constructor and
//                    destructor, so T needs no default constructor
//   LSUP-7, LSUP-10  the placement form and std::construct_at, declared here
//   STG-5, NEW-37    construct_at creates the object, also in constant
//                    evaluation
//   DTOR-6           explicit destructor call, also for int (a scalar)
//   PACK-1, PACK-22, PACK-23  emplace forwards its arguments
//   CE-1, TR-5       the same code runs in static_assert and in main

namespace std {
template <class T, class... Args>
constexpr T* construct_at(T* at, Args&&... args) {
  return ::new (static_cast<void*>(at)) T(static_cast<Args&&>(args)...);
}
}  // namespace std
inline void* operator new(decltype(sizeof 0), void* at) noexcept { return at; }

// One slot of storage. The union has no active member until construct_at
// creates `value`.
template <class T>
union Slot {
  T value;
  constexpr Slot() {}
  constexpr ~Slot() {}
};

template <class T, int Capacity>
  requires (Capacity > 0)
class Stack {
  Slot<T> slots[Capacity];
  int count = 0;

 public:
  constexpr Stack() = default;
  Stack(const Stack&) = delete;
  Stack& operator=(const Stack&) = delete;
  constexpr ~Stack() {
    while (count > 0) pop();
  }

  static constexpr int capacity() { return Capacity; }
  constexpr int size() const { return count; }
  constexpr bool empty() const { return count == 0; }
  constexpr bool full() const { return count == Capacity; }

  // Builds the element in place. Returns false when the stack is full.
  template <class... Args>
  constexpr bool emplace(Args&&... args) {
    if (full()) return false;
    std::construct_at(&slots[count].value, static_cast<Args&&>(args)...);
    ++count;
    return true;
  }
  constexpr bool push(const T& v) { return emplace(v); }

  constexpr T& top() { return slots[count - 1].value; }

  constexpr bool pop() {
    if (empty()) return false;
    --count;
    slots[count].value.~T();
    return true;
  }
};

// A user type with no default constructor.
struct Point {
  int x;
  int y;
  constexpr Point(int a, int b) : x(a), y(b) {}
};

constexpr int int_stack_test() {
  Stack<int, 3> s;
  int result = 0;
  if (!s.push(1)) return -1;
  if (!s.push(2)) return -2;
  if (!s.push(3)) return -3;
  if (s.push(4)) return -4;          // full
  while (!s.empty()) {
    result = result * 10 + s.top();  // 3, 2, 1 in that order
    s.pop();
  }
  if (s.pop()) return -5;            // empty
  return result;                     // 321
}

constexpr int point_stack_test() {
  Stack<Point, 2> s;
  s.emplace(1, 2);
  s.push(Point(10, 20));
  int sum = s.top().x + s.top().y;   // 30
  s.pop();
  sum += s.top().x + s.top().y;      // 33
  return sum;                        // one Point is left; ~Stack ends it
}

static_assert(int_stack_test() == 321);
static_assert(point_stack_test() == 33);
static_assert(Stack<Point, 7>::capacity() == 7);

// A capacity of zero breaks the requires clause. A requires expression in
// a static_assert outside a template would be rejected (CPT-27), so the
// test goes through a concept.
template <int N>
concept StackCapacity = requires { typename Stack<int, N>; };
static_assert(StackCapacity<1>);
static_assert(!StackCapacity<0>);

// A run-time check with a counter, to show that pop and ~Stack each end
// one object.
struct Counted {
  static int live;
  int id;
  explicit Counted(int i) : id(i) { ++live; }
  Counted(const Counted& o) : id(o.id) { ++live; }
  ~Counted() { --live; }
};
int Counted::live = 0;

int main() {
  if (int_stack_test() != 321) return 1;
  if (point_stack_test() != 33) return 2;
  {
    Stack<Counted, 4> s;
    s.emplace(1);
    s.emplace(2);
    s.push(Counted(3));              // the temporary ends at the ';'
    if (Counted::live != 3) return 3;
    s.pop();
    if (Counted::live != 2) return 4;
    if (s.top().id != 2) return 5;
  }                                   // ~Stack ends the last two
  if (Counted::live != 0) return 6;
  return 0;
}
