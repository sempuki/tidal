# The constexpr, consteval, and constinit specifiers

These specifiers control when evaluation occurs.

- `constexpr` on a variable makes it a constant. On a function, it lets
  {term}`constant evaluation` call the function.
- `consteval` makes a function an *immediate function*. Each call must be
  a constant expression.
- `constinit` requires that a variable with static or thread
  {term}`storage duration` has a constant initializer.

Rules are prefixed `CX` for `constexpr`, `CEV` for `consteval`, and
`CINIT` for `constinit`.
