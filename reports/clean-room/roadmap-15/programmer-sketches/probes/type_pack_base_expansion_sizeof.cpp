struct A {}; 
template <class T, class... B>
struct Strong : B... { T v{}; };
static_assert(sizeof(Strong<long long, A>) == sizeof(long long));
int main() { return 0; }
