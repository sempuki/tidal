// Two folds that CPT-19 makes from the same concept in two declarations.
template <class T> concept A = sizeof(T) > 0;
template <class T> concept B = sizeof(T) > 1;
template <A... T> constexpr int f(T...) { return 1; }
template <A... T> requires B<int> constexpr int f(T...) { return 2; }
static_assert(f(1, 2) == 2);
int main() {}
