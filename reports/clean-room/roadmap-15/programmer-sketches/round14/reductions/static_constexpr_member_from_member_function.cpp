template <class T> struct M {
  static constexpr int make() { return sizeof(T); }
  static constexpr int value = make();
};
static_assert(M<int>::value == sizeof(int));
int main() {}
