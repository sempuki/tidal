template <class Sig> struct Signal;
template <class R, class... Args> struct Signal<R(Args...)> {
  template <class T> T collect(T init, Args... args) { return init + (args + ...); }
};
int main() {
  Signal<int(int)> s;
  return s.collect(1, 2) == 3 ? 0 : 1;
}
