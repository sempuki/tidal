struct T { static constexpr int primes[] = {2, 3, 5}; };
int main() { int s = 0; for (int p : T::primes) s += p; return s == 10 ? 0 : 1; }
