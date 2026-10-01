# rustRet

- rust crate for massspectrometry retention time and retention index.
- all massspectrometry claculation in one crate.
- this crate will do also the machine learning on the mass-spectrometry data.

```
cargo build
```

```
_ __   _   _   ___  | |_  |  _ \  | ____| |_   _|
| '__| | | | | / __| | __| | |_) | |  _|     | |  
| |    | |_| | \__ \ | |_  |  _ <  | |___    | |  
|_|     \__,_| |___/  \__| |_| \_\ |_____|   |_|  
                                                 

rustRET.
         ************************************************
         Author Gaurav Sablok,
         Email: codeprog@icloud.com
        ************************************************

Usage: rustRet <COMMAND>

Commands:
retention-index        retention index calculation
time-retention         retention time calculate
retention-time-adjust  retention time adjusted calculation
machinelearning        machine learning metabolomics
help                   Print this message or the help of the given subcommand(s)

Options:
-h, --help     Print help
-V, --version  Print version

```
```
./target/debug/rustRet retention-index ./testfile/sample.txt
elutes	retentiontime
6	678.5714285714286
```
Gaurav Sablok \
codeprog@icloud.com
