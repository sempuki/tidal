// A template-id in the first branch of if constexpr on a concept-id.
template <class T> concept Wide = sizeof(T) > 1;
template <Wide T> struct Box { T v; };
template <class T> int g(T t) {
  if constexpr (Wide<T>) { Box<T> b{t}; return 2; }
  else return 1;
}
int main() { return g('a') == 1 && g(5) == 2 ? 0 : 1; }
