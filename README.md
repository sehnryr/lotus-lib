# lotus-lib

This is a Rust library for reading data from Warframe's files stored in the
`Cache.Windows` folder. Data stored in these cache files contains compressed 
data in a directory-like structure.

## Dependencies

This library depends on `oodle-sys` which is a wrapper around
`liboo2corelinux64.so` which in turn needs to be installed on your system.
You can get it by following the instructions here:
https://github.com/sehnryr/get-oodle-lib

## Credits

This library is based on the work of [LotusLib](https://github.com/Puxtril/LotusLib)
by [Puxtril](https://github.com/Puxtril). The original library is written in C++.
