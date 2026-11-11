template <class T> struct M {
  static constexpr int make() { return sizeof(T); }
  static constexpr int value = make();
};
int main() { return M<char>::value - 1; }
