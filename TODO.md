- Encode tokens as enum and write a main function that runs a hardcoded program
- Parser in main
  - Should we use a library for this later?
- Separate parsing and interpreting into library
- Basic unit and property tests for sample programs, both interpreter and parser
- Should we define a macro for making it easier to write ASTs in tests?
- Basic CLI support
  - Reading from a file
  - Reading from stdin
  - CLI docs
- Setup hax
- Setup Github actions and prevent pushing to main
  - Running hax as pre-merge check
- Implement basic optimizations for strings of data pointer instructions
  - Basically like constant folding for increment/decrement pairs, but maybe we
    can do this a bit more elegantly
  - Like a normalization pass
  - Normalization of loops when applicable (haven't studied the spec yet, I'm
    sure there's some way to analyze this as well)
- Hax proofs of correctness of above optimizations
- Wonder if we could or should implement some constraints to our implementation
  of our language and then a static analysis pass
  - e.g. limiting the amount of data cells, really forcing each cell to only
    store a byte
  - Then I could probably also set up some hax ensure-annotations and 'check
    correctness' of static analysis that way...
  - Actually right this is way trickier than I remembered, since the lang is
    turing complete
  - But there are still some checks I can do I guess, for example that the jump
    commands are balanced always
- This project might be a bit tough to present, but if I could deploy this in
  e.g. Github pages...
  - Thinking about this more, might be interesting to implement a stepping
    interpreter and then also prove that it produces equivalent output when
    compared to the 'regular' interpreter
  - Web UI could then use stepping interpreter for a visualization
