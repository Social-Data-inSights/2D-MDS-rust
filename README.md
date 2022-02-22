# 2D MDS rust

Method to create a webassembly module that take an initialized positions of some point, their distances and apply the mds method from [sklearn MDS](https://scikit-learn.org/stable/modules/generated/sklearn.manifold.MDS.html)

For building the webassembly:
```
wasm-pack build --target web
```

More information on webpack : https://webassembly.org/getting-started/developers-guide/