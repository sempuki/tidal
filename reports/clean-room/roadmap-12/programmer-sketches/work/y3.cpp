template <unsigned N> struct Name { char s[N]; };
template <Name<3> N> int first() { return N.s[0]; }
int main() { return first<Name<3>{{'a', 'b', 0}}>() - 'a'; }
