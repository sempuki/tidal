template <int N> struct Fixed {
  long long raw;
  bool operator<(Fixed o) const;
};
template <int N> bool Fixed<N>::operator<(Fixed o) const { return raw < o.raw; }
template <class T> struct Stats {
  T low;
  bool below(T v) const;
};
template <class T> bool Stats<T>::below(T v) const { return v < low; }
template struct Stats<Fixed<1>>;
int main() {
  Stats<Fixed<1>> s{{5}};
  return s.below(Fixed<1>{3}) ? 0 : 1;
}
