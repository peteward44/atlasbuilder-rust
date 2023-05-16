# atlasbuilder-rust
[![Rust](https://github.com/peteward44/atlasbuilder-rust/actions/workflows/rust.yml/badge.svg)](https://github.com/peteward44/atlasbuilder-rust/actions/workflows/rust.yml)

Free to use command line tool to create [texture atlases](https://en.wikipedia.org/wiki/Texture_atlas) (otherwise known as texture sprites / spritesheets).
Uses max-rects bin packing algorithm to place sub images optimally within the atlas.
Intended to be used as part of an automated tool chain, so is not interactive and only accepts command line parameters.

## Basic usage

Simply specify input filenames on the command line:

```
atlasbuilder my_image.png my_other_image.png
```

Will output out.png (spritesheet image) and out.json (meta data, telling you the positions of the sub images within the atlas)

## Using a different meta data template

Output using a JSON format which outputs as an array instead of a key-value object:

```
atlasbuilder --template json-array my_image.png my_other_image.png
```

TOML format

```
atlasbuilder --template toml my_image.png my_other_image.png
```

YAML format

```
atlasbuilder --template yaml my_image.png my_other_image.png
```

XML format

```
atlasbuilder --template xml my_image.png my_other_image.png
```

## Using a custom meta data template

Templates are stored in the "templates" folder where atlasbuilder is installed. You can add your own file here and specify the name using the ```--template``` argument. You can examine the existing templates to understand how it works.
The templates are defined using the Rust crate [Tera](https://tera.netlify.app/docs#templates)

## --help output

```
Builds texture atlas images with meta data output

Usage: atlasbuilder [OPTIONS] <input>...

Arguments:
  <input>...  Image filenames to add to atlas

Options:
  -r, --rotation-disable
          Disable sub image rotation
  -f, --fixed-size
          Output image will be a fixed width / height instead of attempting to use as little as possible
      --width <width>
          Maximum width of output atlas - must be power of 2 [default: 4096]
      --height <height>
          Maximum height of output atlas - must be power of 2 [default: 4096]
  -o, --image-output <image-output>
          Output filename for .png file [default: out.png]
      --meta-output <meta-output>
          Output filename for meta file [default: ]
  -m, --meta-template <meta-template>
          Template to use for outputted meta information. See docs for details [default: json-hash]
  -p, --padding <padding>
          Pixel padding inbetween subimages [default: 2]
      --input-name-root-dir <input-name-root-dir>
          Root directory to use for all relative input paths in the meta data [default: ]
      --output-name-root-dir <output-name-root-dir>
          Root directory to use for all relative output paths in the meta data [default: ]
  -h, --help
          Print help
  -V, --version
          Print version
```

