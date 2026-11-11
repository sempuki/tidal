template <class T> concept Small = sizeof(T) <= 8;
template <class... T>
  requires (Small<T> && ...)
constexpr int count() { return sizeof...(T); }
int main() { return count<int, char>() - 2; }
