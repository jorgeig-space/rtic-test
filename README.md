# rtic-test
When using a PAC compiled with svd2rust 0.19, the `InterruptNumber` trait is used on interrupts instead of the `Nr` trait that was used before. RTIC is expecting the `Nr` trait, so compilation fails with message:

```
error[E0277]: the trait bound `Interrupt: Nr` is not satisfied
 --> src/main.rs:7:1
  |
7 | #[rtic::app(device = stm32wl::stm32wle5, peripherals = true)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Nr` is not implemented for `Interrupt`
  |
  = note: this error originates in the attribute macro `rtic::app` (in Nightly builds, run with -Z macro-backtrace for more info)
```

# PAC changes in this project

Cargo.toml contains three different PAC implementations coming from the same source but compiled with different `svd2rust` and by different people:

```
# stm32wl compiled with svd2rust 0.17
# stm32wl = { git = "https://github.com/jorgeig-space/stm32wl.git", features = ["stm32wle5", "rt"]}

# stm32wl compiled with svd2rust 0.19
stm32wl = { git = "https://github.com/jorgeig-space/stm32wl-latest.git", features = ["stm32wle5", "rt"] }

# external stm32wl also compiled with svd2rust 0.19
# stm32wl = { git = "https://github.com/newAM/stm32wl-rs.git", rev = "239ceea719f2e26b70e8680f87a31b4a309b5ff8", features = ["stm32wle5", "rt"]}

# a HAL that uses the external stm32wl (svd2rust 0.19)
# [dependencies.stm32wl-hal]
# git = "https://github.com/newAM/stm32wl-hal.git"
# rev = "1e760a889cbddff6ab5ee2cce0a1bf88c723b2bf" # put a specific git commit hash here
# features = ["stm32wle5","rt",]
```
