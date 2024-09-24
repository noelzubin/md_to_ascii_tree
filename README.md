# Markdown lists to ascii tree
Convert markdown lists and nesting to ascii tree. 

Run 
``` sh
$ echo "
* Physical Devices
  * Partition Tables
    * Partitions
          * Physical Volumes (PV)
          * Volume Groups (VG)
            * Logical Volumes (LV)
              * File Systems
                * Virtual File System (VFS)
    * File Systems
      * Virtual File System (VFS)
" | cargo run 

# output
Physical Devices
└── Partition Tables
    ├── Partitions
    │   └── Physical Volumes (PV)
    │       └── Volume Groups (VG)
    │           └── Logical Volumes (LV)
    │               └── File Systems
    │                   └── Virtual File System (VFS)
    └── File Systems
        └── Virtual File System (VFS)
```

Install
```
cargo install --path .
```