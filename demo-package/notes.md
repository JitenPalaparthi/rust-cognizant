- two types of crates
    1. library create
    2. binary create

- to create a pure library crate

    ```
    cargo new --lib demo
    ```
    
    - task

        shapes
            |_ shape
                    |_ rect
                    |_ square
                    |_ cuboid

- create a trait in shape IShape -> area and perimeter as definitions
_ all types like rect, square and cuboid must implement IShape trait

- in main call them using trait