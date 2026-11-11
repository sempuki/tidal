template <class Q, class... Qs>
  requires ((sizeof(Q) == sizeof(Qs)) && ...)
constexpr int count(Q, Qs...) { return 1 + sizeof...(Qs); }
int main() { return count(1, 2, 3) - 3; }
