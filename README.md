# `tomatron` 

<p align="center">
  <img src="portrait.svg" alt="Portrait">
</p>

* **Name:**  `Tomatron Irrigatron`
* **Titul:** `Der Konig alle Tomato`

## Bekante Eigeschaften und Eigenturm: 

``` toml
[material]
wasser = {menge="2.5"}

[pump]
volume = 30

[mictobit]
id="2AKFP"

[mictobit.automationbit]
analog={directory="input"}

```

[micro:bit]:https://github.com/nrf-rs/nrf51-hal
[nrf-rs]:https://github.com/nrf-rs
## Text weiter ist eine Kopie und nur zum info da.. 
> A template for building applications for ARM Cortex-M microcontrollers

This project is developed and maintained by the [Cortex-M team][team].


## `micto:bit`
build &flash.it

```
openocd -f interface/cmsis-dap.cfg -f target/nrf51.cfg

```

```
gdb-multiarch target/thumbv6m-none-eabi/debug/tomatron
.
.
(gdb)target remote :3333
...
(gdb) load
```