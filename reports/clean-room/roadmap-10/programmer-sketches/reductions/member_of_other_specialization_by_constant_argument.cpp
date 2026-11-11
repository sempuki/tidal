// Inside Box<N>, a member of another specialization Box<0> (DEP-15 calls it
// dependent). The front end looks the member up at the definition and rejects.
template <int N> struct Box {
  int v = N;
  int read(const Box<0>& o) const { return o.v; }
};
int main() {
  Box<0> z;
  Box<1> one;
  return one.read(z);
}
