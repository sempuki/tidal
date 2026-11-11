// An argument does not satisfy the concept of a constrained template.
template <class T> concept Hashable = requires(T t) { t.hash(); };
template <Hashable T> int bucket(T t) { return t.hash() % 8; }
struct Key { int k; };
int main() { return bucket(Key{3}); }
