// DEP-5: an independent call keeps the function found at the definition.
int scale(double) { return 1; }
template <class T> struct Holder {
  int get() const { return scale(2); }
};
int scale(int) { return 2; }
int main() { return Holder<char>{}.get() == 1 ? 0 : 1; }
