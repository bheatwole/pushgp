# PushGP
PushGP is a Rust implementation of the Push 3.0 genetic programming language. The main source for the language
definition can be found at http://faculty.hampshire.edu/lspector/push3-description.html

## Push Concepts
Push achieves its combination of syntactic simplicity and semantic power through the use of a stack-based execution architecture that includes a stack for each data type. A CODE data type, with its own stack and an associated set of code-manipulation instructions, provides many of the more interesting features of the language. Push instructions, like instructions in all stack-based languages, take any arguments that they require and leave any results that they produce on data stacks. To provide for "stack safe" execution of arbitrary code Push adopts the convention, used widely in stack-based genetic programming, that instructions requiring arguments that are not available (because the relevant stacks are empty) become NOOPs; that is, they do nothing. Because Push's stacks are typed, instructions will always receive arguments and produce results of the appropriate types (if they do anything at all), regardless of the contexts in which they occur.

## Generating Random Code

## Implementing Mutation

## Implementing Crossover



## Generating a Push Implementation
The point of this library is to create a new stack-based execution architecture that includes stacks, literal values and instructions
within the problem domain you face. This execution architecture is created at compile-time for performance.

## Performance Considerations
Genetic Programming typically runs a huge number of simulations in order to find the optimal solution. It is important that the execution context be as fast
as possible. With Rust trait and generics we create an execution context that pushes as many decisions as possible to the compile-time optimizer. This impacts
the coding process in the following ways:
- The compiler must know the exact list of instructions you wish to use
- The compiler must have a known call-site for each instruction
- The runtime parser must know how to turn text code into a call to one of the instructions
- The runtime parser must know how to an instruction into text code
- The compiler must know the types of each stack in the context
- The compiler must know which stacks support literal values located in the text code
- The runtime parser must know how to parse a literal value from text code
- The runtime parser must know how to turn a literal value into an instruction to push the literal on the appropriate stack
- The runtime parser must know how to turn an instruction to push a literal value back into text code
- The runtime code generator must know the exact list of instructions
- The runtime code generator must know how to generate a new random value



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



Code is the component that knows how to parse strings and turn them into Instructions. It also knows how to create a new
random instruction for the Code type.

An Instruction knows how to execute itself within a virtual machine and may hold data as well. The Instruction trait
meets all the requirements so that Instructions can be used at trait objects.

Some Instructions, when executed, produce a new random Instruction by calling out to its associated Code. An Instruction
depends upon certain traits of the Virtual Machine such that it won't compile without it. (i.e. an instruction requires
the Bool stack to exist, or for certain State trait implementations)

Instructions also know how to turn themselves back to strings.

Both the Code and Exec stacks hold `Box<dyn Instruction>`

The Virtual Machine holds the RNG and the memory (both Stack and State). It is a unique struct created by the user of
this library and, with some helper macros, implements all the traits necessary to execute Instructions.

The weights for random code are stored in the Virtual Machine as well