template <class... Args> struct Sig {
  int collect(int init, Args... args) { return init + (args + ...); }
};
int main() {
  Sig<int> s;
  return s.collect(1, 2) == 3 ? 0 : 1;
}
