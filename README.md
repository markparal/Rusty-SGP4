# Rusty-SGP4
A Rust crate to parse and propagate Two-Line Element (TLE) sets using Simplified Perturbations Models (SGP4).

## Motivations
I am pursuing this project for two reasons
1. To learn to write Rust code
2. To dive into the theory surrounding SGP4

## Plan
- Implement TLE parsing and storage
- Read "Revisiting Spacetrack Report #3: Rev 3"
- Read "History of Analytical Orbit Modeling in the U. S. Space Surveillance System"
- Implement SGP4 algorithm from Hoots et al
- Implement changes from Vallado et al
- Test with test cases from Vallado et al

## TODOs
- Error handling
- Implement SGP4 equations
- Write a math spec
- Include a visualizer?

## Testing and Documentation
```bash
# To run the unit tests
cargo test

# To build the Rust Docs
cargo doc
```

## Resources
[Revisiting Spacetrack Report #3: Rev 3](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev3.pdf)
[Fundamentals of Astrodynamics Github Repository](https://github.com/CelesTrak/fundamentals-of-astrodynamics?tab=readme-ov-file)
[History of Analytical Orbit Modeling in the U. S. Space Surveillance System](https://arc.aiaa.org/doi/abs/10.2514/1.9161?journalCode=jgcd)