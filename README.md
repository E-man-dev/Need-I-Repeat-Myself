# Need I Repeat Myself (NIRM)
An app built with godot-rust for handling repeating patterns in yml configuration, but which is possibly useful elsewhere.

[Installation](##Installation) 
[About NIRM](##About_NIRM) 
[Todo](##Todo) 
[Useful Links](##Useful_Links) 
[Screenshots](##Screenshots)

## Installation
### Prebuilt
Download one of the releases! I plan on supporting the following:

- Windows AMD64
- Linux AMD64 and ARM64
- macOS ARM64

If you cannot find a release for your operating system but your operating system is on the above list, please raise an issue and I will try to get out a release.
For now, I have no idea if anyone is actually gonna use this, so I won't bother with more releases until I know someone wants them.

### Compile it Yourself
If you wish to compile this for yourself, here is how you do it:

Follow https://godot-rust.github.io/book/intro/setup.html to get Godot and godot-rust set up. You may find that you need additional developer tools, in which case set those up as you find you need them.

Clone this repo using `git clone https://github.com/E-man-dev/Need-I-Repeat-Myself.git` or your method of choice.

Navigate to the project's /rust folder in the terminal. Most file managers will let you open the location in a terminal if you right click. Run `cargo build`. If you plan on building a release, run `cargo build --release` as well.

Open the project in Godot by pointing Godot to the .godot file found in the godot folder. With the project open, go to Project > Export. Select the preset for your operating system and run Export Project.
If you see a message complaining about missing export templates, you will need to download those. This is not tied to the project, this is just a thing for Godot.
I allowed Godot to go online to automatically download the templates, but it doesn't make a difference to me if you do it differently.

## About NIRM
### What is NIRM? Why is NIRM?
Need I Repeat Myself, or NIRM, is meant to be functional. I built this because I had a need for it. Why did I build it with Godot? Why did I build it with Rust? Because I wanted to. I had never used either before, but I wanted to give both a try.
Godot does not advise using their game engine for regular app development, and godot-rust will tell you that Rust is a curious choice for developing games.

I mentioned I had a need for NIRM. A friend of mine needed some yml work done for his Minecraft server. The yml had a very distinct pattern, but not one that was a simple copy-paste. Every part had a cost and slot number, but it would alternate between three different variants, each of which had special text that needed to be added. Hence, NIRM was born. If I could simply paste in each of the three parts and replace the cost, slot number, and special text with placeholders, I could then iterate through and replace those placeholders with what actually needed to be there. It needed to be an app with a GUI since I wanted my friend to be able to use the tool, so I decided to give Godot and Rust a try. The rest is history.

### The Logo
![NIRM logo](/godot/icon.svg)

While this logo throws out most advice the [Papirus](https://github.com/PapirusDevelopmentTeam/papirus-icon-theme) team could offer, it does have a very recognizable outline and tells you exactly what the app is.
Each of the four letters of the acronym overlaps with parts of another letters, symbolizing the parts of the text that do repeat.
At the same time, each letter has a part that simply cannot recycle a part of another letter, symbolizing the parts of the text that change each iteration. The colors were chosen for the same reason.
Red, Green, and Blue are each separate colors on standard computer displays, but the grey is a mix of all three.

### Did you use AI?
A little. I would love to claim that I did this all myself, but I got stuck on a few spots. To be clear, I sanity-checked everything the AI spit out and wrote as much as I could by myself, but to say I didn't use AI at all would be lying.

## Todo
- [ ] Add releases for other supported OSes
- [ ] Optimize builds further
- [ ] Correct rust/Cargo.toml
- [ ] Add something for number slots
- [ ] Add tooltip text to all boxes
- [ ] Add screen reader compatibility
- [ ] Add instructions for how to use NIRM

## Useful Links
https://godot-rust.github.io/book/intro/setup.html

https://docs.godotengine.org/en/stable/tutorials/ui/creating_applications.html

## Screenshots
<img width="1412" height="964" alt="Screenshot_Empty_Fields" src="https://github.com/user-attachments/assets/5465376f-4500-4c18-ad98-770a4ac6ae06" />
<img width="2343" height="1893" alt="Screenshot_Sample_In_Action" src="https://github.com/user-attachments/assets/68ed47a2-7a9a-4fa3-b00a-1726fc845cc0" />



I do not claim rights to any trademarks for Windows, macOS, or any others which I do not expressly claim for myself.
