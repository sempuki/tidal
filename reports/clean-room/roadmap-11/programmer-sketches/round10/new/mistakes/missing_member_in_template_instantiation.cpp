// Planted mistake: the template calls size() on a type that has none.
template <class T> int count(const T& t) { return t.size(); }
struct Bag { int n = 3; };
int main() { return count(Bag{}); }
