struct Name { char s[3]; };
template <Name N> int first() { return N.s[0]; }
int main() { return first<Name{{'a', 'b', 0}}>() - 'a'; }
