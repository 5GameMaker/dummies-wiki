# Running UMT on Linux

## Original Windows-only UMT[^1]

> ⚠️
>
> Drag&drop freezes the application!
>
> Use `File`/`Open` or `^o` to open files instead.

Windows GUI binaries can be used on Linux via Wine.

`$ WINE_D3D_CONFIG="renderer=vulkan" wine UndertaleModTool.exe`

Setting renderer to `vulkan` fixes dialog windows being rendered pitch black.

### Fonts

Arial font must be installed, otherwise code editor will crash.

`$ winetricks arial` 

### Dialog window crash

When unfocusing window with a popup open, UMT may crash with `XI_BadDevice`.

To fix this, unset `DISPLAY` or set it to empty: `$ DISPLAY= ..`

## Availonia port[^2]

Download, unpack and run <https://nightly.link/luizzeroxis/UndertaleModTool/workflows/publish_gui_avalonia/avalonia/GUI-ubuntu-latest-Release-isBundled-true-isSingleFile-false.zip>.
This should work without any modifications.

[^1]: [How to run UndertaleModTool on Wine by WilsontheWolf](https://gist.github.com/WilsontheWolf/0eca47bf15f3a1811d03db93af960df5)
[^2]: https://github.com/UnderminersTeam/UndertaleModTool/pull/2126
