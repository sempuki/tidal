# decltype, unevaluated operands, and discarded values

`decltype` gives the type of an expression without evaluating it. Its
operand is one of the *unevaluated operands* of the language, with those
of `sizeof` and `noexcept`. A *discarded-value expression* is evaluated
only for its effects, and its value is not used. This page covers the
type that `decltype` gives, what an unevaluated operand allows, and what
happens to a discarded value.

Depends on: primary expressions, value categories, unary operators.
