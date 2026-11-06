// notes 10 and 11: explicit elements for a pack that is not last, and a deduced parameter after it.
template <class... Ts, class U> constexpr int mid(Ts..., U u) { return int(sizeof...(Ts)) * 10 + int(sizeof(U)); }
int main() { return mid<int>(1, 'c') == 11 ? 0 : 1; }
