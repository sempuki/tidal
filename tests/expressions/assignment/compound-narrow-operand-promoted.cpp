// tidal-rules: ASN-5 CONV-2
// tidal-expect: run
// tidal-note: X17-7. E1 is promoted to int inside op, so E1 op E2 has type int. The result converts back to the type of E1 modulo its range (CONV-2). The short case runs only at run time, because EDG rejects the conversion of 32768 to short during constant evaluation (Q-0124).
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;

constexpr unsigned char byte_sum() {
  unsigned char c = 250;
  c += 10;
  return c;
}
static_assert(byte_sum() == 4);

unsigned char byte = 250;
static_assert(same<decltype(byte + 10), int>);

int main() {
  unsigned char c = 250;
  c += 10;
  if (c != 4) return 1;
  short s = 32767;
  s += 1;
  if (s != -32768) return 2;
  return 0;
}
