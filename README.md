
LP55231
=======

This is driver for the [TI LP55231](http://www.ti.com/product/LP55231) that implements the [`embedded-hal`](https://github.com/rust-embedded/embedded-hal/) traits.

*Note* that this driver is not yet platform-agnostic, as it relies on the `cortex-m` crate for a startup delay.

[Datasheet](http://www.ti.com/lit/gpn/lp55231)

What works
----------

* Turning LEDs on and off
* Resetting the IC

TODO
----

- [ ] add support for writing programs
- [ ] add support for using the temperature sensor
- [ ] add support for GPIOs
