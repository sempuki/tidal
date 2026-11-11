// Planted mistake: a constexpr if branch with a failing static_assert that the
// instantiation reaches.
template <class T> int kind() {
  if constexpr (sizeof(T) == 1) return 1;
  else { static_assert(sizeof(T) == 1, "only bytes"); return 0; }
}
int main() { return kind<int>(); }
