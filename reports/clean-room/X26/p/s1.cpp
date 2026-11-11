// A comma in a default template argument of a function template.
// Today the first template is chosen. If X26:new-1 were a substitution
// failure, the second would be chosen and the result would change.
template <class T, int = (T::value, 0)> int pick(int) { return 1; }
template <class T> int pick(long) { return 2; }
struct A { static const int value = 1; };
int main() { return pick<A>(0) == 1 ? 0 : 1; }
