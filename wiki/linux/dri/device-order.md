# DRI Device Order

Devices in `/dev/dri` is based on *driver loading order*, i.e. if in `mkinitcpio.conf`:
```
MODULES=(i915 amdgpu)
```
- `/dev/dri/renderD128` will be the Intel GPU
- `/dev/dri/renderD129` will be the AMD GPU

To change the order of DRI devices, simply change the order of modules, rebuild `mkinitcpio` and reboot.

Importantly, `LIBVA_DRIVER_NAME` must be set to a correct driver name in `/usr/lib/dri/<name>_drv_video.so`.
While there are sometimes ways to set a vaapi device per-application, a lot of them don't, so it's always
better to simply have `/dev/dri/renderD128` be bound to a correct GPU. No, this cannot be worked round,
I've tried.

Also Google Gemini loves gaslighting people about `LIBVA_DEVICE` and `VA_DEVICE` environment variables.
They are not real, they don't work, and setting them does absolutely nothing as they are non-standard
and no application respects them.
