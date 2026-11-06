// notes 11: a pack before the end takes its explicit elements, and the last parameter pairs after them.
template <class... Ts> constexpr int lead(Ts..., int last) { return int(sizeof...(Ts)) * 10 + last; }
int main() { return lead<char, char>('a', 'b', 3) == 23 ? 0 : 1; }
