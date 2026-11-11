template <class... Args> struct Sig {
  template <class T> T collect(Args... args, T init) { return init + (args + ...); }
};
int main() {
  Sig<int> s;
  return s.collect(2, 1) == 3 ? 0 : 1;
}
