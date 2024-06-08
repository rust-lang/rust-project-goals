# General notes

This is a place for the goal slate owner to track notes and ideas for later follow-up.

Candidate goals:

* Track feature stabilization
* Finer-grained infra permissions
* Host Rust contributor event

## Areas where Rust is best suited and how it grows

Rust offers particular advantages in two areas:

* **Latency sensitive or high scale network services**, which benefit from Rust’s lack of garbage collection pauses (in comparison to GC’d languages).
* **Low-level systems applications**, like kernels and embedded development, benefit from Rust’s memory safety guarantees and high-level productivity (in comparison to C or C++).
* **Developer tooling** has proven to be an unexpected growth area, with tools ranging from IDEs to build systems being written in Rust.

### Who is using Rust

Building on the [characters from the async vision doc](https://rust-lang.github.io/wg-async/vision/characters.html), we can define at least three groups of Rust users:

* **Alan**[^kay], an experienced developer in a Garbage Collected language, like Java, Swift, or Python.
    * Alan likes the idea of having his code run faster and use less memory without having to deal with memory safety bugs.
    * Alan's biggest (pleasant) surprise is that Rust's type system prevents not only memory safety bugs but all kinds of other bugs, like null pointer exceptions or forgetting to close a file handle.
    * Alan's biggest frustration with Rust is that it sometimes makes him deal with low-level minutia -- he sometimes finds himself just randomly inserting a `*` or `clone` to see if it will build -- or complex errors dealing with features he doesn't know yet.
* **Grace**[^hopper], a low-level, systems programming expert.
    * Grace is drawn to Rust by the promise of having memory safety while still being able to work "close to the hardware".
    * Her biggest surprise is cargo and the way that it makes reusing code trivial. She doesn't miss `./configure && make` at all.
    * Her biggest frustration is 
* **Barbara**[^liskov]

[^kay]: In honor of Alan Kay, inventor of [Smalltalk](https://en.wikipedia.org/wiki/Smalltalk), which gave rise in turn to Java and most of the object-oriented languages we know today.
[^hopper]: In honor of Grace Hopper, a computer scientist, mathematician, and rear admiral in the US Navy; inventor of [COBOL](https://en.wikipedia.org/wiki/Grace_Hopper).
[^liskov]: In honor of Barbara Liskov, a computer science professor at MIT who invented of the [CLU](https://en.wikipedia.org/wiki/CLU_(programming_language) programming language.

### How Rust adoption grows

The typical pattern is that Rust adoption begins in a system where Rust offers particular advantage. For example, a company building network services may begin with a highly scaled service. In this setting, the need to learn Rust is justified by its advantage. 

Once users are past the initial learning curve, they find that Rust helps them to move and iterate quickly. They spend slightly more time getting their program to compile, but they spend a lot less time *debugging*. Refactorings tend to work "the first time".

Over time, people wind up using Rust for far more programs than they initially expected. They come to appreciate Rust's focus on reliability, quality tooling, and attention to ergonomics. They find that while other languages may have helped them edit code faster, Rust gets them to *production* more quickly and reduces maintenance over time. And of course using fewer languages is its own advantage.

### How Rust adoption stalls

Anecdotally, the most commonly cited reasons to stop using Rust is a feeling that development is "too slow" or "too complex". There is not any one cause for this.

* **Language complexity:** Most users that get frustrated with Rust do not cite the borrow checker but rather the myriad workarounds needed to overcome various obstacles and inconsistencies. Often "idomatic Rust" involves a number of crates to cover gaps in core functionality (e.g., `anyhow` as a better error type, or `async_recursion` to permit recursive async functions). Language complexity is a particular problem
* **Picking crates:** Rust intentionally offeres a lean standard library, preferring instead to support a rich set of crates. But when getting started users are often overwhelmed by the options available and unsure which one would be best to use. Making matters worse, Rust documentation often doesn't show examples making use of these crates in an effort to avoid picking favorites, making it harder for users to learn how to do things.
* **Build times and slow iteration:** Being able to make a change and quickly see its effect makes learning and debugging effortless. Despite our best efforts, real-world Rust programs do still have bugs, and finding and resolving those can be frustratingly slow when every change requires waiting minutes and minutes for a build to complete.

### Additional concerns faced by companies

For larger users, such as companies, there are additional concerns:

* **Uneven support for cross-language invocations:** Most companies have large existing codebases in other languages. Rewriting those codebases from scratch is not an option. Sometimes it possible to integrate at a microservice or process boundary, but many would like a way to rewrite individual modules in Rust, passing data structures easily back and forth. Rust's support for this kind of interop is uneven and often requires knowing the right crate to use for any given language.
* **Spotty ecosystem support, especially for older things:** There are a number of amazing crates in the Rust ecosystem, but there are also a number of notable gaps, particularly for older technologies. Larger companies though often have to interact with legacy systems. Lacking quality libraries makes that harder.
* **Supply chain security:** Leaning on the ecosystem also means increased concerns about supply chain security and business continuity. In short, crates maintained by a few volunteers rather than being officially supported by Rust are a risk. 
* **Limited hiring pool:** Hiring developers skilled in Rust remains a challenge. Companies have to be ready to onboard new developers and to help them learn Rust. Although there are many strong Rust books available, as well as a number of well regarded Rust training organizations, companies must still pick and choose between them to create a "how to learn Rust" workflow, and many do not have the extra time or skills to do that.
