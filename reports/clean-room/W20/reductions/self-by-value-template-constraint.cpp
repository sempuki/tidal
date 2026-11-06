// A constructor template Box(T) whose constraint copies a T. Inside a
// requirement, Box(t) with t of type const Box& considers the template
// with T = Box. CPY-15 says such a specialization never gives a
// constructor. GCC 16 and Clang 22 accept and exit 0. The front end
// reports: rejected [CNS-35] at line 9.
struct Box {
  Box() = default;
  template <class T>
    requires requires(const T& t) { T(t); }
  Box(T) {}
};
template <class T>
concept Boxable = requires(const T& t) { Box(t); };
static_assert(Boxable<Box>);
int main() { return 0; }
