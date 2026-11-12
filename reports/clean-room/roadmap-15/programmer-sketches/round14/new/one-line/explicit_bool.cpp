template <class T> struct Box {
  T v;
  template <class U> explicit(sizeof(U) > sizeof(T)) constexpr Box(U u) : v(static_cast<T>(u)) {}
};
template <class T, class U> concept implicit_from = requires(U u) { [](Box<T>) {}(u); };
static_assert(implicit_from<long long, int>);
static_assert(!implicit_from<char, long long>);
int main() { Box<long long> b = 3; Box<char> c(65); return static_cast<int>(b.v) + c.v - 68; }
