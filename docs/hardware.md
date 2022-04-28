## Hardware Requirements

See [Polkadot's requirement for running validators](https://wiki.polkadot.network/docs/maintain-guides-how-to-validate-polkadot)

But for testing purposes, we can use lower requirements.

#### Intel CPU
* Intel(R) Core(TM) i3-4160 CPU @ 3.60GHz - works well
* Intel(R) Core(TM) Xeon CPU @ 2.20 GHz - not recommended. This CPU cannot use Rust to compile the latest version of this codebase properly

#### Apple M1 CPU
* Apple's M1 CPUs have no reported problem running this codebase

#### Hard Disk
The codebase itself is about 200 MB, but the compiled code in the target folder will be around 19 GB. Thus the total required harddisk size should be at least 20 GB.

#### Memory
* 16 GB RAM
