template <class T> concept Small = requires(T t) { t + t; requires sizeof(T) <= 4; };
static_assert(Small<int> && !Small<double>);
int main() {}
