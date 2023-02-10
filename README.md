![YouTube Channel Subscribers](https://img.shields.io/youtube/channel/subscribers/UCXgqRZv7bHsKzwYBrtA9DFA?label=Youtube%20Subscribers&logo=Alaydriem&style=flat-square)

<div align="center">

  <h1>Bedrock Material List</h1>

  <a href="https://www.youtube.com/@Alaydriem"><img src="https://raw.githubusercontent.com/alaydriem/bedrock-material-list/master/docs/subscribe.png" width="140"/></a>

  <p>
    <strong>Generates a simple material list from a Minecraft Bedrock .mcstructure file.</strong>
  </p>
  <hr />
</div>

## Can I get a material list?

It's the number one comment and question that appears on my Youtube channel, and as far as I am aware, Minecraft Bedrock doesn't provide any way to generate this. For larger builds it's not feasible to manually count everything, and using destructive techniques and counting is prone to mistakes and errors.

We have .mcstructure blocks though. Can we extract the information from that?

Yes, the answer is yes.

This rust application does one simple thing - takes a .mcstructure file from Minecraft Bedrock, and turns it into a very rudimentary and simple JSON formatted materials list.

## Usage

```
cargo install bedrock-material-list
edrock-material-list structure --file /path/to/file.mcstructure
```

Or clone the repo, then run it with your .mcstructure file:

```
cargo run -- structure --file /path/to/file.mcstructure
```

The application will output a very simple JSON list that you can use to to generate a materials list for your build.

## How can I make this better?

Subscribe to my Youtube channel.Pull requets are welcome. Here's some features that would be awesome to have:

1. Translate `minecraft:` notation into the actual block name.
2. Consolodate various `minecraft:` states into single blocks: (eg `minecraft:redstone_torch`, `minecraft:unlit_redstone_torch`, `minecraft:lit_restone_torch` are the same item for testing purposes)
3. Create a .mcstructure file that has all the block in game so that it can be used for testing and so we can compare the output with. (multiple states would be great too!)
4. Figure out how to include entities and entity container items as part of the output list! (eg chests, hoppers, dispensers, droppers each have their own entity list and count).
5. Generate an image to use in tutorials that contains the block photo and the block count.
6. And anything else that would make generating a Minecraft Bedrock materials list easier.