# Arrays - Feature Documentation

## Overview
Arrays have been added to the Butter language, allowing you to store and manipulate collections of values.

## Syntax

### Array Type Declaration
```butter
Array<ElementType>
```

### Creating Arrays

**Empty array (using array_new function):**
```butter
let mut numbers: Array<Int> = array_new();
```

**Array literal:**
```butter
let numbers: Array<Int> = [1, 2, 3, 4, 5];
let names: Array<String> = ["Alice", "Bob", "Charlie"];
let scores: Array<Float> = [98.5, 87.3, 92.1];
```

### Accessing Array Elements
```butter
let first: Int = numbers[0];
let second: Int = numbers[1];
```

### Modifying Arrays

**Setting values (requires mutable array):**
```butter
let mut arr: Array<Int> = [1, 2, 3];
arr[0] = 10;  // arr is now [10, 2, 3]
```

**Adding elements:**
```butter
let mut arr: Array<Int> = array_new();
array_push(arr, 42);
array_push(arr, 100);
```

## Complete Examples

### Example 1: Basic Array Usage
```butter
fn main() => nil {
    // Create an array literal
    let numbers: Array<Int> = [10, 20, 30, 40, 50];
    
    // Access elements
    let first: Int = numbers[0];
    let third: Int = numbers[2];
    
    println("First element: ");
    println(first);
    println("Third element: ");
    println(third);
}
```

### Example 2: Mutable Arrays
```butter
fn main() => nil {
    let mut scores: Array<Int> = [85, 90, 78];
    
    println("Original first score: ");
    println(scores[0]);
    
    // Modify an element
    scores[0] = 95;
    
    println("Updated first score: ");
    println(scores[0]);
}
```

### Example 3: Building Arrays Dynamically
```butter
fn main() => nil {
    let mut collection: Array<Int> = array_new();
    
    array_push(collection, 1);
    array_push(collection, 2);
    array_push(collection, 3);
    
    println("First element: ");
    println(collection[0]);
    println("Second element: ");
    println(collection[1]);
    println("Third element: ");
    println(collection[2]);
}
```

### Example 4: Arrays with Loops
```butter
fn main() => nil {
    let numbers: Array<Int> = [1, 2, 3, 4, 5];
    let mut i: Int = 0;
    let mut sum: Int = 0;
    
    while i < 5 {
        sum += numbers[i];
        i += 1;
    }
    
    println("Sum: ");
    println(sum);
}
```

### Example 5: Arrays as Function Parameters
```butter
fn printFirst(arr: Array<Int>) => nil {
    println("First element: ");
    println(arr[0]);
}

fn main() => nil {
    let data: Array<Int> = [100, 200, 300];
    printFirst(data);
}
```

### Example 6: String Arrays
```butter
fn main() => nil {
    let greetings: Array<String> = ["Hello", "Hola", "Bonjour"];
    
    println(greetings[0]);
    println(greetings[1]);
    println(greetings[2]);
}
```

## Array Functions

### `array_new() => Array<T>`
Creates a new empty array.
```butter
let mut arr: Array<Int> = array_new();
```

### `array_push(arr, element) => nil`
Adds an element to the end of the array.
```butter
array_push(arr, 42);
```

### `array_get(arr, index) => T`
Gets an element at the specified index (called automatically with `arr[i]` syntax).
```butter
let value: Int = arr[0];  // Uses array_get internally
```

### `array_set(arr, index, value) => nil`
Sets an element at the specified index (called automatically with `arr[i] = value` syntax).
```butter
arr[0] = 100;  // Uses array_set internally
```

## Important Notes

1. **Array indices start at 0** - The first element is at index 0
2. **Bounds checking** - Accessing an index outside the array will cause a runtime panic
3. **Type safety** - All elements in an array must be the same type
4. **Mutable vs Immutable** - Use `mut` keyword to create mutable arrays that can be modified
5. **Arena allocation** - Arrays use the arena allocator, so no manual memory management needed

## Under the Hood

Arrays are implemented as:
- Dynamic arrays that can grow as needed
- Stored as void pointers internally for type flexibility
- Backed by arena allocation for automatic memory management
- Include bounds checking for safety

## Testing Your Array Code

Create a file `test_arrays.butter`:
```butter
fn main() => nil {
    println("Testing arrays...");
    
    let nums: Array<Int> = [1, 2, 3, 4, 5];
    println("Array created with 5 elements");
    
    println(nums[0]);
    println(nums[4]);
    
    println("Array tests passed!");
}
```

Compile and run:
```bash
cargo run run test_arrays.butter
```

## Future Enhancements

Potential future features:
- `array_len(arr)` - Get array length
- `array_pop(arr)` - Remove last element
- `array_clear(arr)` - Remove all elements
- Array slicing
- Multi-dimensional arrays
