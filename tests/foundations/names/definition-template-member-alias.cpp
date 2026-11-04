// tidal-rules: NAM-21
// tidal-expect: run
// tidal-note: The member alias of the class template is named without qualification after Counter<T>::add, in the parameter list and in the body.
template <class T> struct Counter {
  using value_type = T;
  value_type count = 0;
  value_type add(value_type step);
};
template <class T> T Counter<T>::add(value_type step) {
  value_type before = count;
  count = before + step;
  return count;
}
int main() {
  Counter<long> c;
  c.add(3);
  return c.add(4) == 7 ? 0 : 1;
}
