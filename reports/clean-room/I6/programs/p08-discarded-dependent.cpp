// IF-13, DEP-28: a discarded branch with a dependent error is not instantiated.
struct Small { char c; };
template <class T> int pick() {
  if constexpr (sizeof(T) > 4) {
    return T::nothing;
  } else {
    return 1;
  }
}
int main() { return pick<Small>() == 1 ? 0 : 1; }
