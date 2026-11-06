// Subsumption stands in for the check of P23 form 1. The caller's constraints
// are P, the callee's are Q. A call that picks the P overload shows that P is
// more constrained than Q.
template <class T> concept Wide = sizeof(T) > 1;
template <class T> concept Small = Wide<T> && sizeof(T) < 16;
template <class T> concept Even = sizeof(T) % 2 == 0;
template <class T> requires Wide<T> int q1() { return 1; }                 // Q: Box needs Wide
template <class T> requires Small<T> int q1() { return 2; }                // P: caller has Small
template <class T> requires (sizeof(T) > 1) int q2() { return 1; }         // Q: same text, other place
template <class T> requires Small<T> int q2() { return 2; }
template <class T> requires Wide<T> int q3() { return 1; }
template <class T> requires Even<T> int q3() { return 2; }                 // P: Even does not imply Wide by identity
int a = q1<int>();
#ifdef BAD2
int b = q2<int>();
#endif
#ifdef BAD3
int c = q3<int>();
#endif
int main() { return a == 2 ? 0 : 1; }
