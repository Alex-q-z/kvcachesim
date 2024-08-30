# KVCacheSim

## Run some tests

There are some tests/scripts under `kvcachesim/src/bin/`. Under `kvcachesim/` or `kvcachesim/src/`, you can try running
```
cargo run --bin [TEST_NAME]
```

## DAM documentation

Clone [the original DAM repo](https://github.com/stanford-ppl/DAM-RS). Then, run
```
cargo +nightly doc
```
The index page would be located at `DAM-RS/target/doc/dam/all.html`.

## Reference

You can refer to [the DAM paper](https://alex-q-z.github.io/files/dam-isca24.pdf) (published in ISCA 2024).
```bibtex
@inproceedings{zhang2024dataflow,
  title={The Dataflow Abstract Machine Simulator Framework},
  author={Zhang, Nathan and Lacouture, Rubens and Sohn, Gina and Mure, Paul and Zhang, Qizheng and Kjolstad, Fredrik and Olukotun, Kunle},
  booktitle={2024 ACM/IEEE 51st Annual International Symposium on Computer Architecture (ISCA)},
  pages={532--547},
  year={2024},
  organization={IEEE}
}
```
