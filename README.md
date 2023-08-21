# Animal Facts

An API server using Axum to return random animal facts.

My main focus on this project was to make adding further animal APIs as easy as possible - see [Adding new animals](#adding-new-animals) for a short tutorial.

## Configuration

You can configure the server address and port by editing the [configuration file](./fact-settings.toml)

```toml
port = 3000
addr = "127.0.0.1"
```

In the same file, you can also configure the fact sources for each animal. See [Adding new animals](#adding-new-animals).

## Running the server

Just run cargo on it

```bash
cargo run --release
```

## Consuming the API

The server have only one endpoint, `/fact`.
A `GET` request to `/fact` will return a random fact about any supported animal.

```bash
curl '127.1:3000/fact'
```

You can also ask for a specific animal:

```bash
curl '127.1:3000/fact?animal=cat'
curl '127.1:3000/fact?animal=dog'
```

### Response format

```json
{
    "animal": "cat",
    "fact": "a random fact about cats"
}
```

## Adding new animals

Adding new animals is fairly easy, you need to edit just two file, in four steps to add support for a new animal.

Lets add support for facts about horses!

1. Edit the configuration file [`./fact-settings.toml`](./fact-settings.toml)

Under sources, add the animal name and the API from which we will get the facts.

```toml
[sources]
horse = "https://cat-fact.herokuapp.com/facts/random?animal_type=horse&amount=1"
```

2. Add the new animal to the enum

Open the [`animal.rs`](./src/animal.rs) and add the new variant to the `Animal` enum.

```rust
pub enum Animal {
    Cat,
    Dog,
+   Horse,
}
```


3. Create the intermediate deserialization struct

`animal-facts` will try to parse the original API response into a struct you provide before converting it to the `Fact` type.
The struct must implement `serde::Deserialize` and `Into<Fact>`.

```
#[derive(Deserialize)]
struct HorseFact {
    text: String,
}

impl From<HorseFact> for Fact {
    fn from(horse: HorseFact) -> Fact {
        Fact::new(Animal::Horse, horse.text)
    }
}
```

4. Add the intermediate struct to the animal map

Now we just need to make the application aware of horses now.
Still on the [`animal.rs`](./src/animal.rs), add a new entry to the animal index, mapping the `Animal::Horse` to its handler.

```rust
pub fn init_animal_index(config: HashMap<Animal, String>) -> eyre::Result<()> {
    // ...
    let index = IndexMap::from([
        (
            Animal::Cat,
            make_animal_info::<CatFact>(try_animal_from_config(&config, Animal::Cat)?),
        ),
        (
            Animal::Dog,
            make_animal_info::<DogFact>(try_animal_from_config(&config, Animal::Dog)?),
        ),
+       (
+           Animal::Horse,
+           make_animal_info::<HorseFact>(try_animal_from_config(&config, Animal::Horse)?),
+       ),
    ]);
    // ...
}
```

and that's it, now you can get facts about horses with `curl '127.1:3000/fact?animal=horse'`.
