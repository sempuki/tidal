// tidal-rules: DEP-33
// tidal-expect: run
template <class... T> int count(T...) {
  (void(sizeof(void(T))), ...);
  return sizeof...(T);
}
int main() { return count(); }
