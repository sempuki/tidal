// tidal-rules: TSP-24
// tidal-expect: run
// tidal-note: thread_local is the one storage class specifier that an explicit specialization can have.
template <class T> thread_local int counter = 0;
template <> thread_local int counter<char> = 1;
int main() { return counter<char> == 1 && counter<int> == 0 ? 0 : 1; }
