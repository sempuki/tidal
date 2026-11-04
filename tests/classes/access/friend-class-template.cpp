// tidal-rules: ACC-20
// tidal-expect: run
// Each specialization of Box befriends every specialization, so
// Box<long> reads the private member of a Box<int>.
template <class T> class Box {
  T value;
  template <class U> friend class Box;
public:
  explicit Box(T v) : value(v) {}
  template <class U> Box(const Box<U>& other) : value(other.value) {}
  T get() const { return value; }
};
int main() {
  Box<int> small(7);
  Box<long> wide(small);
  return wide.get() == 7 ? 0 : 1;
}
