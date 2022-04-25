# PushGP
PushGP is a Rust implementation of the Push 3.0 genetic programming language. The main source for the language
definition can be found at http://faculty.hampshire.edu/lspector/push3-description.html

## Push Concepts
Push achieves its combination of syntactic simplicity and semantic power through the use of a stack-based execution architecture that includes a stack for each data type. A CODE data type, with its own stack and an associated set of code-manipulation instructions, provides many of the more interesting features of the language. Push instructions, like instructions in all stack-based languages, take any arguments that they require and leave any results that they produce on data stacks. To provide for "stack safe" execution of arbitrary code Push adopts the convention, used widely in stack-based genetic programming, that instructions requiring arguments that are not available (because the relevant stacks are empty) become NOOPs; that is, they do nothing. Because Push's stacks are typed, instructions will always receive arguments and produce results of the appropriate types (if they do anything at all), regardless of the contexts in which they occur.

## Generating Random Code

## Implementing Mutation

## Implementing Crossover



## Generating a Push Implementation
The point of this library is to create a new stack-based execution architecture that includes stacks and instructions
within the problem domain you face.

## Implementing Custom Instructions
A macro or proc_macro wrapper around a fn. The wrapper generates an instruction struct from the fn. The types of the
parameters to the fn determine which stacks are popped into order to call the fn. The generated code includes parsing
and display for the instruction. It also include an option state-like parameter that enables the fn to modify the
state of an individual.

## Implementing Custom Stacks
If implementing a DNA search, you might want a stack representing the four basic nucleotides. If implementing a stock
algorithm, you might want a stack representing the stocks you are tracking. If implementing a game AI, you might want a
stack for enemies and a stack for friendlies.

While custom types could be implemented simply as items on the Integer stack, it does make it harder for the algorithm
to find a viable individual because the elements will be mixed with other Integers. For example, if the top integer is
`3`, is this `IBM` or 'take the three-day moving average'?

This is accomplished with a macro that takes a series of types as parameters and produces both a generated Context and
a generated Configuration.