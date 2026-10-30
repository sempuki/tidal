// tidal-rules: IF-13
// tidal-expect: run
template <class T>
int size_of(const T& t) {
  if constexpr (requires { t.size(); })
    return t.size();
  else
    return 1;
}
struct Box { int size() const { return 4; } };
int main() { return size_of(Box{}) + size_of(3) == 5 ? 0 : 1; }
