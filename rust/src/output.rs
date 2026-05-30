use godot::prelude::*;
use godot::classes::{CodeEdit, ICodeEdit, SpinBox};

//Creates a structure for any Output to follow
#[derive(GodotClass)]
#[class(base=CodeEdit)]
pub struct Output {
    base: Base<CodeEdit>,
}

//How to implement the Output structure. The Output box still needs its node type to be Output for any of this to do anything.
#[godot_api]
impl ICodeEdit for Output {
    fn init(base: Base<CodeEdit>) -> Self {
         Self {
             base,
         }
    }
}

//The logic behind the output box. If the code documentation style is not kosher, deal with it. This is a personal project.
#[godot_api]
impl Output {
    #[func]
    pub fn on_button_pressed(&mut self) {
        /*
         * My strategy is to first just pull in the values of stuff, so let's start there
         */

        /*
         * Repetitions
         */
        // SpinBox
        let sbv = self.base().get_node_as::<SpinBox>("../../../VBoxContainerL/HBoxContainer/SpinBox");
        let sbvalue = sbv.get_value();

        /*
         * Text blocks
         */
        // FirstBlockText
        let fbt = self.base().get_node_as::<CodeEdit>("../../../VBoxContainerL/FirstBlockText");
        let fbtext = fbt.get_text();
        // SecondBlockText
        let sbt = self.base().get_node_as::<CodeEdit>("../../../VBoxContainerL/SecondBlockText");
        let sbtext = sbt.get_text();
        // ThirdBlockText
        let tbt = self.base().get_node_as::<CodeEdit>("../../../VBoxContainerL/ThirdBlockText");
        let tbtext = tbt.get_text();

        /*
         * Flags
         */
        // Yes, I am inconsistent about calling them flags vs placeholders
        // SlotNumberFlag
        let snf = self.base().get_node_as::<CodeEdit>("../HBoxContainer/SlotNumberFlag");
        let snflag = snf.get_text();
        // CostNumberFlag
        let cnf = self.base().get_node_as::<CodeEdit>("../HBoxContainer/CostNumberFlag");
        let cnflag = cnf.get_text();
        // SpTextFlag1
        let st1f = self.base().get_node_as::<CodeEdit>("../HBoxContainer2/SpTextFlag1");
        let st1flag = st1f.get_text();
        // SpTextFlag2
        let st2f = self.base().get_node_as::<CodeEdit>("../HBoxContainer2/SpTextFlag2");
        let st2flag = st2f.get_text();
        // SpTextFlag3
        let st3f = self.base().get_node_as::<CodeEdit>("../HBoxContainer2/SpTextFlag3");
        let st3flag = st3f.get_text();

        /*
         * Replacements
         */
        // SlotNumbers
        let snt = self.base().get_node_as::<CodeEdit>("../SlotNumbers");
        let sntext = snt.get_text();
        // CostNumbers
        let cnt = self.base().get_node_as::<CodeEdit>("../CostNumbers");
        let cntext = cnt.get_text();
        // FirstSpText
        let s1t = self.base().get_node_as::<CodeEdit>("../FirstSpText");
        let s1text = s1t.get_text();
        // SecondSpText
        let s2t = self.base().get_node_as::<CodeEdit>("../SecondSpText");
        let s2text = s2t.get_text();
        // ThirdSpText
        let s3t = self.base().get_node_as::<CodeEdit>("../ThirdSpText");
        let s3text = s3t.get_text();

        /*
         * Separator for special text
         */
        // SpTextSeparator
        let sst = self.base().get_node_as::<CodeEdit>("../SpTextSeparator");
        let sstext = sst.get_text();

        /*
         * Next, the Replacements are not actually usable until they are an array of strings, so we convert them to that.
         */

        /*
         * Conversion of replacements. Matthias was already converted, so the apostles didn't have to do that.
         */
        let sntextarr = sntext.split(", ");
        let cntextarr = cntext.split(", ");
        let s1textarr = s1text.split(&sstext);
        let s2textarr = s2text.split(&sstext);
        let s3textarr = s3text.split(&sstext);

        /*
         * And now, at long last, the loop that does it all.
         */
        let mut current_text = 1;
        let mut current_iter = 0;
        let mut current_sn = 0;
        let mut current_cn = 0;
        let mut current_s1 = 0;
        let mut current_s2 = 0;
        let mut current_s3 = 0;
        let mut iter_out: GString;
        let mut output_text = GString::from("");

        if sbvalue == 0.0 && fbtext == GString::from("") && sbtext == GString::from("") && tbtext == GString::from("") {
            self.base_mut().set_text("All the fields on the left side are not optional. Don't you have things you are trying to modify?");
        }
        else if sbvalue == 0.0 {
            self.base_mut().set_text("Please indicate a number of iterations. Zero won't do you much good.");
        }
        else if fbtext == GString::from("") && sbtext == GString::from("") && tbtext == GString::from("") {
            self.base_mut().set_text("Fill in at least one block text so you have something to iterate through.");
        }
        else {
            while current_iter < sbvalue as i32 {
                if current_text == 1 {
                    iter_out = fbtext.replace(&st1flag, &s1textarr[current_s1]);
                    if current_s1 < s1textarr.len() - 1 {
                        current_s1 = current_s1 + 1;
                    }
                    else {
                        current_s1 = 0;
                    }
                    current_text = 2;
                }
                else if current_text == 2 {
                    iter_out = sbtext.replace(&st2flag, &s2textarr[current_s2]);
                    if current_s2 == s2textarr.len() - 1 {
                        current_s2 = 0;
                    }
                    else {
                        current_s2 += 1;
                    }
                    current_text = 3;
                }
                else {
                    iter_out = tbtext.replace(&st3flag, &s3textarr[current_s3]);
                    if current_s3 == s3textarr.len() - 1 {
                        current_s3 = 0;
                    }
                    else {
                        current_s3 += 1;
                    }
                    current_text = 1;
                }

                // To not confuse the user, we don't count iterations where there is no text to modify. While the special text isn't sensitive to that kind of thing, slot and cost are.
                if iter_out != GString::from("") {
                    iter_out = iter_out.replace(&snflag, &sntextarr[current_sn]);
                    if current_sn == sntextarr.len() - 1 {
                        current_sn = 0;
                    }
                    else {
                        current_sn += 1;
                    }

                    iter_out = iter_out.replace(&cnflag, &cntextarr[current_cn]);
                    if current_cn == cntextarr.len() - 1 {
                        current_cn = 0;
                    }
                    else {
                        current_cn += 1;
                    }

                    current_iter = current_iter + 1;

                    output_text = output_text.insert(output_text.len(), &iter_out);
                }
            }

            self.base_mut().set_text(&output_text);
        }
    }
}

// "Thus saith the Lord of hosts, the God of Israel, unto all that are carried away captives, whom I have caused to be carried away from Jerusalem unto Babylon;
// Build ye houses, and dwell in them; and plant gardens, and eat the fruit of them;
// Take ye wives, and beget sons and daughters; and take wives for your sons, and give your daughters to husbands, that they may bear sons and daughters; that ye may be increased there, and not diminished.
// And seek the peace of the city whither I have caused you to be carried away captives, and pray unto the Lord for it: for in the peace thereof shall ye have peace.
// For thus saith the Lord of hosts, the God of Israel; Let not your prophets and your diviners, that be in the midst of you, deceive you, neither hearken to your dreams which ye cause to be dreamed.
// For they prophesy falsely unto you in my name: I have not sent them, saith the Lord."
// Jeremiah 29:4-9
//
// The question that many Christians struggle who desire to live a godly life is whether anything short of being a missionary or a pastor or a full-time evangelist is pleasing to God. What about hobbies? Do I need to put those aside? This passage from Jeremiah is what I come back to. I can have hobbies for the glory of God. What I mean is, I developed this code to help a friend. To maybe help you too.
