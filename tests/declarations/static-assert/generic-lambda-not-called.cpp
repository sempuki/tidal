// tidal-rules: SA-5
// tidal-expect: accept
// tidal-note: The call operator of a generic lambda is a template. Its body is a templated context, and no call instantiates it, so the static assertion is not checked.
auto never = [](auto) { static_assert(sizeof(int) == 0); };
