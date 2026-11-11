template <class M> struct traits { using cls = int; };
template <class T> int call(typename traits<T>::cls x) { return x; }
int main() {
  return call<char>(4) == 4 ? 0 : 1;
}
