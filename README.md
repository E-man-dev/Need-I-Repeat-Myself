# Need-I-Repeat-Myself
App built with godot-rust for handling repeating patterns in yml configuration, but possibly useful elsewhere.

# Todo
Add license
Fix godot/NIRM.gdextension libraries for anything that is not arm linux
Add releases
Optimize builds further
Correct rust/Cargo.toml
Add something for number slots
Add tooltip text to all boxes
Add screen reader compatibility
Add instructions for installing/compiling
Explain the logo, explain the app's existence

# Things I found helpful
https://docs.godotengine.org/en/stable/tutorials/ui/creating_applications.html


# Installation
Download one of the releases! If you cannot find a release for your device, but your device is on the list of devices I plan to support, please raise an issue and I will try to get out a release. For now, I have no idea if anyone is actually gonna use this, so I won't bother with more releases until I know someone wants them.
Follow https://godot-rust.github.io/book/intro/setup.html to get Godot and godot-rust set up. Clone this repo. Navigate to the rust folder in the terminal. Run cargo build. If you plan on building a release, append --release so that it reads cargo build --release.
Open the project in Godot by pointing Godot to the .godot file found in the godot folder.
If you have not already, you will need to download the export templates. This is not tied to the project, this is just a thing for Godot. I allowed Godot to go online to automatically download the templates, but it doesn't make a difference to me if you do it differently.
 https://docs.godotengine.org/en/stable/tutorials/ui/creating_applications.html#optimizing-distribution-size
