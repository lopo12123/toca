### toca

[repository](https://github.com/lopo12123/toca) |
[issue](https://github.com/lopo12123/toca/issues) |
[mail](mailto:lopo@zju.edu.cn?subject=about20%toca)

A library for record/display keyboard and mouse actions in a period of time.

- `record`: Continuously record mouse/keyboard behavior until a preset `stop-key` is detected.
- `display`: Plays a sequence of mouse/keyboard behavior events of the specified type and position(position is just for
  mouse event).

You can find all the usage in the `examples` folder.


> CHANGELOG

- `v0.1.0`
    - record mouse event.
    - display mouse event.
    - record keyboard event.
    - display keyboard event.
- `v0.1.1`
    - support `from_string` to generator action from `json`.
    - support `to_string` to serialize action to `json`.