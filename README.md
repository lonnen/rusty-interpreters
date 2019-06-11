In the spirit of [Elf Strenberg's simple bytecode interpreters series](https://elfsternberg.com/2019/04/17/worlds-simplest-bytecode-interpreter/), this repo will contain my own work through [Vladimir Kazanov's simple bytecode interpreters](https://github.com/vkazanov/bytecode-interpreters-post/blob/master/README.org).


## VM01 - World's Simplest Bytecode Interpreter

Around 25 LoC with just one register and two instructions. It's not turing complete but it does have all the expected parts. This is a bit simpler than Kazanov's c version because it leverages underlying Rust features. Memory is zero'd automatically and the compiler ensures there are no bad opcodes. It's a cute toy.

## VM02 - World's Second Simplest Bytecode Interpreter

Forked from VM01. This adds arguments to opcodes following Sternberg's articles and differing considerably from Kazanov's implementation. Kazanov relies on C enums being integers under the hood, which is undefined in the C++03 spec and deferred to the compiler. Subtle errors could lead values to be interpreted as instructions and vice-versa.

In Rust, though, the enum is a sum type in Rust it can carry information! In this implementation the bytecode is defined with the integral right in there and avoids the problem.

Of course, this is a contrived issue from the start because this occupies a space between the simplest VM and the simplest practical VM. We still basically have two instructions (Inc and Dec are specialized versions of the two general opcodes) and one register. This was mostly useful as a personal exploration of Rust enums.

## VM03 - Stack Machine

Forked off VM02. We move away from a single register to a stack which can contain several values. Opcodes now assume and operate off a stack. We introduce some new concepts here, including:

* Error States!
* Rust Macros!
* Matches within Matches

We're starting to encounter problems that we cannot check for ahead of time, and those errors need to be detected and propagated. This changes the overall signature of the VM and is reflected in the tests.

A macro is taken from Sternberg to reduce the boilerplate of each opcode. Neat!

Finally this relies heavily on `match` and `Some()`, nesting match statements and generally avoiding `if`. This is a stylistic decision to avoid some `let` sugar. I find it to be easier to read because we're acting on both outcomes and if I understand the sugar correctly it is best used when you only care about one of the two outcomes of `Some()`. In other c-style languages I would eschew the whole mess in favor of returning as early as possible. I'm not sure if that's idiomatic in rust, though, and as a learning exercise it was good to work through both versions of the code.

## VM04 - Register Machine 

The fourth VM examines an alternative to the stack machine: the register machine. This is similar to the approach taken by Dalvik and Lua. This required throwing out the old instruction set in favor of fewer instructions generalized over the register space. 

Kazanov points out that in practice, register-based VMs often end up emulating a stack anyway. Compilers for these machines are more complex for having to decide how to use the available registers. This implementation doesn't address this complexity at all in order to keep the implementing small. 

The tests call some of error states we're not handling. Since this is a toy and the fixes for these errors are straightforward, they are left an exercise. A bounds checking macro is one place to start. I suspect this is also a good candidate for fuzzing with quick check.

## VM05 - Regex lite

The fifth VM is a significant departure from the previous four. It's specialized towards interpreting regular expressions, specifically it should be able to satisfy all of Kleene's Regular Expressions without a closure (*). 4 codes and less than 50 lines accomplish a reasonably capable VM here that uses recursion instead of explicit state. Despite its size, it can express many useful regular expressions over ascii.

This will not support non-ascii very will despite ostensibly operating over unicode because many `char`s in Rust may be necessary to handle Burmese or Korean, for example. This also uses backtracking and is not necessarily suitable for streaming large strings.

If you're interested in more about regular expression engines in Rust, have a look at Sternbeg's [Rigged Regular Expressions](https://github.com/elfsternberg/riggedregex). For the general problem of implementing regular expression engines Kazanov points to [a set of resources maintained by Russ Cox](https://swtch.com/~rsc/regexp/).

## Future VMs

Kazanov expanded from the original set of VMs in to a series. A natural next step from here could be following along Part 2, covering [optimizing bytecode interpreters](https://badootech.badoo.com/when-pigs-fly-optimising-bytecode-interpreters-f64fb6bfa20f), and Part 3, covering [regex bytecode interpreters](https://badootech.badoo.com/regex-bytecode-interpreter-looking-for-needles-in-session-haystacks-9bbff9db09bc) in detail.

This project started as an introduction to Rust features necessary to understand an [LC-3 VM coming from C++](https://justinmeiners.github.io/lc3-vm/), and another direction would be moving from here to build a practical and usable VM with a [complex programs](https://github.com/rpendleton/lc3-2048) [readily available](https://github.com/justinmeiners/lc3-rogue).
