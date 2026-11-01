// tidal-rules: TSP-24 TSP-25
// tidal-expect: run
// tidal-expect-edg: reject
// tidal-error-edg: thread-local declaration follows
// tidal-note: The explicit specialization is thread_local and the template is not. EDG rejects the difference (Q-0203).
template <class T> int counter = 0;
template <> thread_local int counter<char> = 1;
int main() { return counter<char> == 1 ? 0 : 1; }
