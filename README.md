## Usage

My flow is as follows:

  * Edit from a (lightweight) editor at the host machine
  * Go to a console to compile / run / run tests
  * Rinse and repeat
  
To start the container (with the volume):

```bash
make docker-build
```

Once you're inside the container:

```bash
rustc --version
cargo build; cargo run
```
