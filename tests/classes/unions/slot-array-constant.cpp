// tidal-rules: UNI-14
// tidal-expect: run
// tidal-note: A fixed-capacity stack of a type with no default constructor. Point has no default constructor and a destructor that is not trivial, so Slot declares its own constructor and destructor (UNI-3, UNI-11).
inline constexpr void* operator new(decltype(sizeof 0), void* place) noexcept { return place; }
namespace std {
template <class T, class... Args>
constexpr T* construct_at(T* location, Args&&... args) {
  return ::new (static_cast<void*>(location)) T(static_cast<Args&&>(args)...);
}
}
template <class T, int Capacity>
class FixedStack {
  union Slot {
    T value;
    constexpr Slot() {}
    constexpr ~Slot() {}
  };
  Slot slots[Capacity];
  int count = 0;
public:
  constexpr FixedStack() {}
  constexpr ~FixedStack() { while (count > 0) pop(); }
  template <class... Args>
  constexpr void push(Args&&... args) {
    std::construct_at(&slots[count].value, static_cast<Args&&>(args)...);
    ++count;
  }
  constexpr void pop() { --count; slots[count].value.~T(); }
  constexpr T& top() { return slots[count - 1].value; }
};
struct Point {
  int x, y;
  constexpr Point(int a, int b) : x(a), y(b) {}
  constexpr ~Point() {}
};
constexpr int use() {
  FixedStack<Point, 4> stack;
  stack.push(1, 2);
  stack.push(3, 4);
  stack.pop();
  stack.push(5, 6);
  return stack.top().x * 10 + stack.top().y;
}
static_assert(use() == 56);
int main() { return use() == 56 ? 0 : 1; }
