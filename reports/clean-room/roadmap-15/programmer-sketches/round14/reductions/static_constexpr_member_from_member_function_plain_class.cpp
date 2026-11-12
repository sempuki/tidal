struct M {
  static constexpr int make() { return 4; }
  static constexpr int value = make();
};
int main() { return M::value - 4; }
