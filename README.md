# dot_vox_amethyst

![](https://img.shields.io/crates/v/dot_vox_amethyst.svg)
![](https://travis-ci.org/davidedmonds/dot_vox_amethyst.svg?branch=master)
![](https://docs.rs/dot_vox_amethyst/badge.svg)

Rust library for loading [MagicaVoxel](https://ephtracy.github.io/) .vox files into the
[Amethyst](https://amethyst.rs) game engine.

## Current status

Able to load the first model contained within the file, and render it using a flat renderer
with absolutely no shading.

## RustDoc

Kindly hosted over at https://docs.rs/dot_vox_amethyst/.

## Not yet implemented

* Simple lighting
* Shadows
* Physical rendering using material data

## Thanks

As a maintainer, its always nice to get bug reports and (even better) pull requests. Thanks
will follow for any submitted issue or PR.

## See also

* [dot_vox](https://github.com/davidedmonds/dot_vox) - crate for loading `.vox` files using
  nom. 