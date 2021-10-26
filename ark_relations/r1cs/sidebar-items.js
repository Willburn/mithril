initSidebarItems({"enum":[["ConstraintSystemRef","A shared reference to a constraint system that can be stored in high level variables."],["OptimizationGoal","Defines the parameter to optimize for a `ConstraintSystem`."],["SynthesisError","This is an error that could occur during circuit synthesis contexts, such as CRS generation, proving or verification."],["SynthesisMode","Defines the mode of operation of a `ConstraintSystem`."],["TracingMode","Instructs `ConstraintLayer` to conditionally filter out spans."],["Variable","Represents the different kinds of variables present in a constraint system."]],"macro":[["info_span","Constructs a span at the info level."]],"struct":[["ConstraintLayer","A subscriber `Layer` that enables capturing a trace of R1CS constraint generation."],["ConstraintMatrices","The A, B and C matrices of a Rank-One `ConstraintSystem`. Also contains metadata on the structure of the constraint system and the matrices."],["ConstraintSystem","An Rank-One `ConstraintSystem`. Enforces constraints of the form `⟨a_i, z⟩ ⋅ ⟨b_i, z⟩ = ⟨c_i, z⟩`, where `a_i`, `b_i`, and `c_i` are linear combinations over variables, and `z` is the concrete assignment to these variables."],["ConstraintTrace","A captured trace of `tracing` spans that have `target = \"r1cs\"`."],["LcIndex","An opaque counter for symbolic linear combinations."],["LinearCombination","A linear combination of variables according to associated coefficients."],["Namespace","A namespaced `ConstraintSystemRef`."],["TraceStep","A step in the trace of a constraint generation step."]],"trait":[["ConstraintSynthesizer","Computations are expressed in terms of rank-1 constraint systems (R1CS). The `generate_constraints` method is called to generate constraints for both CRS generation and for proving."],["Field","The interface for a generic field."],["ToConstraintField","Types that can be converted to a vector of `F` elements. Useful for specifying how public inputs to a constraint system should be represented inside that constraint system."]],"type":[["Matrix","A sparse representation of constraint matrices."],["Result","A result type specialized to `SynthesisError`."]]});