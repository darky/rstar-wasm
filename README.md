# rstar-wasm

R-tree, which compiled to WebAssembly from rstar crate

## Basic usage

```javascript
import {RTree} from 'rstar-wasm';

const tree = new RTree();

tree.insert([1, 1]);
tree.insert([2, 2]);

tree.nearest([2.2, 2.2]); // [2, 2]

tree.destroy();
```
