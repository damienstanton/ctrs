//! ## or, an exploration of category theory for _systems programmers_
//!
//! ![a category](https://upload.wikimedia.org/wikipedia/commons/f/ff/Category_SVG.svg)
//!
//! What follows in this library is derived from _Category Theory for Programmers_, a long-running
//! blog series by [Bartosz Milewki](https://twitter.com/BartoszMilewski). A nice "book" created
//! from original blog posts can be found [here](https://github.com/hmemcpy/milewski-ctfp-pdf).
//!
//! #### Crate Health
//!
//! [![Build Status](https://travis-ci.org/damienstanton/ctrs.svg?branch=master)](https://travis-ci.org/damienstanton/ctrs)
//!
//! ### Goals
//!
//! My intention is simply to share my learning experience with category theory by using the
//! built-in documentation and testing faculties that Rust provides. I will also conduct screen
//! casts to explore each implementation so that I can make sure that what I commit to the crate is
//! logical. I hope that by doing this, others can apply this knowledge to what they do at `$WORK`
//! in C++, Go, Java, etc.
//!
//! ### Table of Contents
//! | CTFP Chapter  | Topic | Lecture Videos | Notes |
//! | -- | --- | --- | --- | --- |
//! | 1 | Introduction | [Motivation](https://youtu.be/I8LbkfSSR58), [What is a category?](https://youtu.be/p54Hd7AmVFU) |See [`id`](./fn.id.html) and [`compose`](./fn.compose.html)|
//! | 2 | Types & Functions | [Functions, epimorphisms](https://youtu.be/O2lZkr-aAqk), [Monomorphisms, simple types](https://youtu.be/NcT7CGPICzo) | | See //TODO: doc links |
//! ### Non-code challenge questions:
//!
//! Chapter 1
//!
//! _Is the world-wide web a category in any sense? Are links morphisms?_
//!
//! > I would say yes. We know that web pages have something akin to an identity morphism: its
//!  URI/URL. And links between pages may be composable (a link from site A to B can, through the
//!  redirect protocol, map to a third side C).
//!
//! Update: After a conversation with a few people in the #categorytheory channel on FP Slack, care
//! must be taken to specify that we mean the morphism that defines the whole REST or HATEOAS
//! command cycle for a link in this example; not the links themselves. So the correct answer to
//! Bartosz's question depends on what we mean by what `links` are.
//!
//! _Is Facebook a category, with people as objects and friendships as morphisms?_
//!
//! > Not really, because social relationships cannot always compose. Friend C, friend of B, is not necessarily friend of A.
//!
//! _When is a directed graph a category?_
//!
//! > A DAG would classify as a category when a graph _G_ has vertices _V_ and edges _E_ such that:
//! > - all paths in the graph can be concatenated
//! > - each V has an E that loops back to itself (so that it satisfies identity)
//!
//! Chapter 2
//!
//! // TODO: q/a

/// Identity is a unit under composition.
///
/// # Overview
/// We describe a function over a generic type `T` that simply returns its parameterized value,
/// unchanged. This might seem a little odd.
///
/// But as Bartosz describes, the motivation for understanding identity is to enable a higher order
/// of composition:
/// > You might be asking yourself the question: Why would anyone bother
/// > with the identity function — a function that does nothing? Then again,
/// > why do we bother with the number zero? Zero is a symbol for nothing.
/// > Ancient Romans had a number system without a zero and they were
/// > able to build excellent roads and aqueducts, some of which survive to
/// > this day.
/// > Neutral values like zero or id are extremely useful when working
/// >
/// > with symbolic variables. That’s why Romans were not very good at algebra, whereas the Arabs and the Persians, who were familiar with the
/// > concept of zero, were. So the identity function becomes very handy as
/// > an argument to, or a return from, a higher-order function. Higher order
/// > functions are what make symbolic manipulation of functions possible.
/// > They are the algebra of functions.
/// >
/// > To summarize: A category consists of objects and arrows (mor phisms). Arrows can be composed, and the composition is associative.
/// >
/// > Every object has an identity arrow that serves as a unit under composition.
///
/// # Example
/// ```
/// use ctrs::id;
///
/// let x = 1;
/// assert_eq!(id(1), 1);
///
/// let y = "OK";
/// assert_eq!(id(y), "OK");
/// ```
pub fn id<T>(x: T) -> T {
    x
}

/// Composition is the heart of categorical computation.
///
/// # Overview
/// Our definition of composition may appear convoluted, but let's break it down. We start by
/// defining generic types for our two input functions. These are`F` and `G`, respectively. These
/// have a `'static` lifetime because we have to ensure that the borrow checker does not let these
/// types out of scope before computation has finished. Next, we have types `Fv` and `Gv`, which
/// represent the types for the return values for each of the functions F and G. Finally, we have
/// our output type `V`, which is the result we want. We pass the functions F and G as parameters
/// `f` and `g`.
///
/// Next, the return value is a `Box` of the generic `Fn` type that takes an Fv to a V. We have
/// to _box_ the return value because we do not know how much size it could occupy on the stack
/// (thus we allocate to the heap). Finally, we implement trait bounds on F and G, specifying how
/// the chain should compose: F takes an Fv to a Gv, and then G takes a Gv to V.
///
/// Let's now see how this looks in practice using an example.
///
/// # Example
/// ```
/// use ctrs::{id, compose};
///
/// // Let's first define a trivial incrementer function.
/// fn inc(x: i32) -> i32 {
///   x + 1
/// }
///
/// // and cover our bases by confirming inc works as expected.
/// let x = 1;
/// assert_eq!(inc(x), 2);
///
/// // Since we are composing functions on a given value, the syntax is
/// // compose(A, B)(V). Knowing this, our passing test looks like:
/// assert_eq!(compose(id, inc)(1), 2);
/// ```
///
/// We can extend this idea! Let's take the situation where we've also defined an admittedly
/// contrived `double` function, and want to compose its behavior with our existing incrementer.
/// Mathematicians sometimes call the composition operator one might find in Haskell _after_, and
/// understanding the way in which the function associates is indeed _g after f_.
/// ```
/// # use ctrs::{id, compose};
/// # fn inc(x: i32) -> i32 {
/// #   x + 1
/// # }
/// fn double(x: i32) -> i32 {
///    x * 2
/// }
///
/// let x = 1;
/// assert_eq!(compose(inc, double)(1), 4);
/// ```
pub fn compose<F: 'static, G: 'static, Fv, Gv, V>(f: F, g: G) -> Box<dyn Fn(Fv) -> V>
where
    F: Fn(Fv) -> Gv,
    G: Fn(Gv) -> V,
{
    Box::new(move |x| g(f(x)))
}

/// a polymorphic function from any type to the unit type
pub fn unit<T>(_t: T) -> () {
    ()
}

// TODO: ch 2 implementations...
// -----------------------------

// # Memoization (in terms of set theory)
// - A relation is just a subset of pairs
// - These set relations forms a cartesian product
// - So by definition, any subset of the cartesian product _is_ a relation
// - Relations do not have directionality, but functions do
// - A function can therefore be said to be a relation with directional constraints, namely those
// going from a domain (lhs) to a codomain (rhs)
// The following structure holds for invertible functions
// ```haskell
// f :: a -> b
// g :: b -> a
// g after f = id
// f after g = id
// ```
// This geometric understanding helps intuit a meaning for isomorphisms (and this is for
// isomorphisms in any given category.)