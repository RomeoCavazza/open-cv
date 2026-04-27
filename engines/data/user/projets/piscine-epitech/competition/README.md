<!-- markdownlint-disable MD033 -->
<div align="center">
  <img src="../assets/epitech_logo.png" alt="Epitech Logo" width="400" />
  <br />
  <img src="https://img.shields.io/badge/Seminar-5G_Algorithms-6366f1?style=for-the-badge" alt="Seminar Badge" />
  <img src="https://img.shields.io/badge/Final-Competition-00f2a6?style=for-the-badge" alt="Days Badge" />
  <img src="https://img.shields.io/badge/Tech-CUDA_&_Optimization-ff4757?style=for-the-badge" alt="Focus Badge" />
</div>
<!-- markdownlint-enable MD033 -->

# Code Competition: 5G or not 5G?

A high-stakes algorithmic sprint: solving the multi-dimensional challenge of optimal 5G antenna placement with cost efficiency and massive scale in mind.

---

> [!IMPORTANT]
> **The Challenge**: 
> - **Input**: City maps with building coordinates and varying populations.
> - **Hardware**: Diverse 5G antenna types with specific costs and ranges.
> - **Constraint**: 100% population coverage at the **absolute minimum cost**.

## Technical Core

| Layer | Implementation |
|---|---|
| **Logic** | ![Python 3](https://img.shields.io/badge/Language-Python_3-3776AB?style=flat-square&logo=python&logoColor=white) |
| **GPU** | ![CUDA](https://img.shields.io/badge/Acceleration-NVIDIA_CUDA-76B900?style=flat-square&logo=nvidia&logoColor=white) ![Numba](https://img.shields.io/badge/Compiler-Numba_/_JIT-blue?style=flat-square) |
| **Math** | ![NumPy](https://img.shields.io/badge/Crunching-NumPy-013243?style=flat-square&logo=numpy&logoColor=white) ![CuPy](https://img.shields.io/badge/Crunching-CuPy-4EAA25?style=flat-square) |
| **Vis** | ![Matplotlib](https://img.shields.io/badge/Visuals-Matplotlib-11557c?style=flat-square) |

### Heuristic Optimization Logic

```mermaid
graph TD
    Data[Load Maps & Costs] --> Cluster[Spatial Building Clustering]
    Cluster --> GPU[GPU Matrix: Building-Antenna Distances]
    GPU --> Greedy[Greedy Solver: Best Ratio Selection]
    Greedy --> Back[Backtracking & Local Refresh]
    Back --> Result{100% Covered?}
    Result -- No --> Greedy
    Result -- Yes --> Final[Optimized JSON Result]
```

---

## The Implementation (god_tier_cuda.py)

- **Vectorized Pre-computation**: Using NumPy/CuPy to calculate millions of building-antenna distance pairs in milliseconds.
- **Weighted Greedy Heuristic**: Selecting positions based on dynamic `Potential Population Coverage / Installation Cost` ratios.
- **CUDA JIT Acceleration**: Critical kernels implemented with **Numba** to leverage thousands of GPU cores for distance matrices.
- **Local Perturbation**: Fine-tuning the final placement to eliminate redundant antennas.

---

## Skills developed

- **Algorithmic Mastery**: Mastering combinatorial optimization and high-level heuristics.
- **GPU Engineering**: Bridging Python logic with high-performance CUDA kernels.
- **Complex Modeling**: Translating business cost-constraints into mathematical objective functions.
- **Performance at Scale**: Designing systems capable of processing thousands of buildings in seconds.
