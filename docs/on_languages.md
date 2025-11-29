## SOME LANGUAGE IMPRESSIONS
I've used different languages for the Advent of code puzzles, and in some cases it was the best 
way to learn a new language by writing a program that was not a simple tutorial.

This is not enough to see a language real use in a "reasonably sized" project, however, programs 
aren't trivial, and there's enough meat to get to a basic opinion on the language.

I'll write some personal notes on each language used below.
> (updated december 2024)


### Python (professional experience, 5 years+)
I won't hide that python is one of my favourite languages. The syntax is elegant and it's 
incredibly expressive. It might lack a lot of nuances that other languages offer, but it's the 
perfect example of a great 80/20 language: for 80% of uses is perfect, simple to use, nice to 
write, clean to read.
For the extra 20% instead ... it's problematic

The standard library is rich enough and there was no need to install extra dependencies to solve 
the advent of code puzzle. This is a plus when the basic library offer a great toolkit to write 
programs without needing more dependencies.

Weaknesses: 
- for the advent of code, none, some puzzle might take a few minutes to run, but it's acceptable
- however, package and dependencies management is lacking sorely - at the time of writing 
  there's a lot of competing tools trying to gain traction. The major issue is ensuring the 
  legacy works, which sadly is a bit of a mess.
- multithreading and multiprocessing is also a big problem, the experiments in removing the GIL 
  is promising, but we'll see where it goes in the coming year.
- The lack of interfaces reflects the historical period where this language was conceived. These 
  days, where interfaces are preferred to class hierarchies, there's a set of dependencies and 
  kludges to add which clash with the usual terseness of python.


### Rust (new language)
The language is extremely complex, for example the borrow checker is not an easy thing to grok.
However, once it's understood, the language is fantastic and the standard library is incredibly 
expressive; the amount of functional constructs available allow for incredibly powerful processing.
The package manager and the library ecosystem is also very well done; some puzzles required 
dependencies, installing them was easy and worked with no issues.

Traits are a fantastic constructs, the ability to implement nearly any trait for everything is 
way better than C++ operator overloading, and it generally is more understandable. 

Weaknesses:
- borrow checker and lifetimes are the hardest thing to understand. 
- Some simple constructs that other language implement natively or very simple, become more 
  complex in Rust (for example, an hashmap of objects indexed over each object's "name" - ie 
  string) requires a copy (for safety) of the name.
- Lifetimes are very hard to understand and may result in very complex function signatures that 
  can't be read very well. I think I haven't yet found a use for them.
- As much as the compiler is always very expressive and able to infer some things you are 
  attempting, there are some syntactic things that feel cargo cult stuff (I'm referring to 
  returning `Box<dyn Trait>` and `impl Trait`), sometimes Rust seems ok to infer and shorten 
  things, but in other areas it looks extremely verbose.
- Wrapping and propagating errors isn't always easy - there isn't a standard library primitive 
  to create an error much like other Exception based languages have a "generic exception" to 
  throw. I get the reason, avoiding a catch all which is an antipattern, but the convenience for 
  small programs would be useful.


### Go (new language)
The language has some very neat syntax and in general is pleasant to write. Although it feels 
odd at times: it has a simplified C-like syntax, a python-like standard library with decent 
amount of good and powerful basic constructs, but the java-like naming convention and the use of 
uppercase/lowercase naming for the visibility feels ... odd.

The interfaces are great, it's basically like the traits in Rust (which I love) and make for a 
good "encapsulation" without forcing you down the alley of classes and hierarchies which can 
lead to many antipatterns.

Weaknesses:
- the recent addition of generics added some power to the language, however the standard 
  library lacks some decent "functional" construct (like map/filter/... in python).
- I needed a priority queue and the library gives you the code to copy and adapt to your case, 
  it works, but I'm wondering why the language doesn't offer a genericized type...
- the lack of overloading causes function exported by modules to explode in number. The language 
  suggest naming things correctly, and names are generally clear. Go forces our naming muscles 
  to flex, it might be right (if you don't find a name for what you are doing, maybe you're 
  making a mistake), however something like default method parameters (which exist in python 
  and Kotlin for example), would be useful
- I still haven't understood the dependency management, the need for an "url" is weird - I get 
  that it's the easiest thing to implement namespacing and ownership.
- the existence of pointers is great, although it can trip you when implementing interfaces 
  (where you end up modifying a copy of the object and question yourself why it's not working)
- error handling is very C-like and can lead to verbose programs. In general Go is quite verbose,
  more than it looks like on the surface.


