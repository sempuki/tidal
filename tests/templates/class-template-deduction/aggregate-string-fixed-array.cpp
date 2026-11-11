// tidal-rules: CTAD-9 CTAD-10
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: no instance of constructor "Named" matches
// tidal-note: The member char v[3] does not depend on T, and "ab" initializes it whole. 1 gives T = int. EDG forms no viable candidate (Q-0277).
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Named { char v[3]; T n; };
Named b{"ab", 1};
static_assert(same<decltype(b), Named<int>>);
