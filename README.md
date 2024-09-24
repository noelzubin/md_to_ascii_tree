# Markdown lists to ascii tree
Convert markdown lists and nesting to ascii tree. 

Run 
``` sh
$ echo "Root
  Folder1
    Subfolder1
    Subfolder2
  Folder2
    Subfolder3" | cargo run 

# output
 Root
 ├─ Folder1
 │  ├─ Subfolder1
 │  └─ Subfolder2
 └─ Folder2
    └─ Subfolder3
```

Install
```
cargo install --path .
```