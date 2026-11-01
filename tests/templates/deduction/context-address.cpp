// tidal-rules: DED-30
// tidal-expect: accept
// tidal-note: The function type of the template is matched against the target's function type, without the adjustments of a call.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
template <class T> Tag<T> take(T);
template <class T> Tag<T> fwd(T&&);
template <class T> Tag<T> cref(const T&);
Tag<int> (*a)(int) = take;
Tag<long> (*b)(long) = &take;
Tag<int&> (*c)(int&) = fwd;
Tag<int> (*d)(int&&) = fwd;
Tag<int> (&e)(const int&) = cref;
