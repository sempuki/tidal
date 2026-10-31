// tidal-rules: PRIM-9
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-note: Q-0033. The assertion holds for either answer, so only an implementation that refuses the comparison rejects it. Clang refuses it; GCC and EDG fold it.
static_assert("same" != "same" || "same" == "same");
