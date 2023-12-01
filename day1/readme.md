# [Day1](https://adventofcode.com/2023/day/1/)

## Part1

Just filtering for a digit from both ends of a line and add them.

## Part 2

### O(n^2) solution

1. Create a map of string to digit (map of "one" = 1, "two" = 2)
2. Create a reverse map for the right end ("eno" = 1, "owt" = 2)
3. Left, Iterate from i to i+5, if the concatenated string can exist in map 1 or found a digit, then return left
4. Right, Iterate from i to i-5, if the concatenated string can exist in map 1 or found a digit, then return right
5. Left * 10 + Right

### O(nLog(n)) solution

1. Create a graph of string to digit ("o" -> "n" -> "e" = 1)
2. Do the same, but reverse for right
3. Left,
    1. Iterate a line, traverse the graph by each character
    2. if the character isn't a children of the current node, try again from the root, if still not found, ignore
    3. if the character reach the last node, then that's the number, return
    4. if the character is a digit then return

#### Tips
    Instead of having a containins nodes as children, use a flatten hash map instead HashMap<char, Node>. If a char exist in 
    `children`, then look it up from the hashmap.

    ```rust
        struct Node {
            char: char
            value: u32
            children: HashSet<char>
        }
    ```