// tidal-rules: TPL-48
// tidal-expect: accept
// tidal-note: The friend function declaration uses the name of the template parameter and takes the class as a parameter. The template is never instantiated, so the test checks the definition alone. EDG rejects such a friend only when the class is instantiated (Q-0198).
template <class F> struct Box { friend int F(Box); };
