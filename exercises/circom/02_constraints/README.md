# Constraints

Because in Circom we use R1CS Arithmetization, we always have to define our circuits as a system of quadratic equations. And what this means is that we can only have one multiplication per constraint, so if we have an expression with multiple multiplications, we should break it down into multiple quadratic constraints.
