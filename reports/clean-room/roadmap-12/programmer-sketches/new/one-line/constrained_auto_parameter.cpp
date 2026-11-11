// A constrained placeholder parameter picks an overload.
template <class T> concept Small = sizeof(T) <= 2;
int pick(Small auto) { return 1; }
int pick(auto) { return 2; }
int main() { return pick('a') + pick(1.0) - 3; }
