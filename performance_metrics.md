# Performance Comparison: load.py vs. Load Function in lib.rs

|                    |        Rust       |       Python      |       Improvement       |
|:------------------:|:-----------------:|:-----------------:|:-----------------------:|
|    Elapsed Time    | 0.0138063 seconds | 0.0033090 seconds |     x 4.17 (Python)     |
|    Memory Usage    |    0 kilobytes    | 851968 kilobytes  |     x 851968 (Rust)     |


Overall it appears like in this case, Python actually ran faster but had a significantly higher memory usage than Rust for the transform and load functionality. This could be because the code for Rust was not exactly an apples-to-apples match with the existing code for Python and also because there could be differences in advantages between the two languages based on how they are being used (in this case, we were loading data from a locally saved csv to SQLite).

<img width="1036" alt="Screenshot 2024-10-28 at 10 41 19 PM" src="https://github.com/user-attachments/assets/2121c140-3ee0-4e6f-a033-0c5ae022ebeb">



