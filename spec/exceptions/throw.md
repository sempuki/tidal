# Throw expressions and the exception object

A `throw` expression leaves the current point of execution with a value.
The value becomes the exception object, which lives until the last
handler for it finishes. This page covers the operand of `throw`, how the
exception object is made from it, its type and lifetime, `throw` with no
operand, and `throw` during constant evaluation.

Depends on: value categories and temporaries, copy and move, constant
evaluation, objects and lifetime.
