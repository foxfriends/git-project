use cursive::{Cursive, views::*};

pub fn show(siv: &mut Cursive) {
    let help_text = LinearLayout::vertical()
        .child(TextView::new("?: Show this help"));

    let dialog = Dialog::around(help_text)
        .title("Help")
        .button("Ok", |s| { s.pop_layer(); });

    siv.add_layer(dialog);
}
