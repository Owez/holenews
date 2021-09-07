# HoleNews

Automated reporting and wiki website for the [Foxhole](https://store.steampowered.com/app/505460/Foxhole/) game

## Running

Once you have installed Rust, compile with the following command:

```shell
# build in release mode
cargo build --release
```

The outputted binary will be in the `target/release/` directory. Once you have fetched that, please install [`sqlx-cli`]() and run the migrations as such:

```shell
cargo install sqlx-cli
sqlx database create
sqlx migrate run
```

Now all that's left is optional [configuration](#configuration) and then you can move your binary and database to whatever location is most suitable.

## Configuration

The following environment variables may be changed in order to configure an instance:

```env
DATABASE_URL=sqlite://holenews.db
LOG_TRACE=yes
```

<!-- Just some decoration -->
<p align="center">
    <br><br>
    <img src="static/img/heatmap_bg.png" width=500/>
</p>
