struct ToInt { operator int&(); };
template <class S> concept makes = requires(S s) { static_cast<int&>(s); };
int main() { return makes<ToInt> ? 0 : 1; }
