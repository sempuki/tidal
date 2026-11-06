template <auto N> constexpr auto twice = N + N;
template <auto... Vs> constexpr int count = sizeof...(Vs);
int main() { return twice<21> == 42 && twice<1L> == 2L && count<1, true, 3u> == 3 ? 0 : 1; }
