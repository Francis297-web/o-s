//! NEXORA OS
//! splash.rs
//!
//! Controls the startup splash animation.

use std::{thread, time::Duration};

const LETTER_DELAY: u64 = 180;
const GLOW_DELAY: u64 = 250;
const FRAME_DELAY: u64 = 120;

/// Called by the bootloader
pub fn display() {

    clear();

    type_logo();

    glow_logo();

    aurora_ring();

    butterfly();

    tagline();

    loading();
}

/// ======================================================
/// STEP 1
/// Type NEXORA one letter at a time
/// ======================================================
fn type_logo() {

    let word = "NEXORA";

    let mut current = String::new();

    for c in word.chars() {

        current.push(c);

        clear();

        center("");

        center("");
        center(&current);

        thread::sleep(Duration::from_millis(LETTER_DELAY));
    }

}

/// ======================================================
/// STEP 2
/// Make the word glow
/// ======================================================
fn glow_logo() {

    for _ in 0..5 {

        clear();

        center("");

        center("✨ NEXORA ✨");

        thread::sleep(Duration::from_millis(GLOW_DELAY));

        clear();

        center("");

        center("NEXORA");

        thread::sleep(Duration::from_millis(GLOW_DELAY));

    }

}

/// ======================================================
/// STEP 3
/// Aurora Ring
/// ======================================================
fn aurora_ring() {

    let frames = [

r#"
      .
"#,

r#"
     ***
   *     *
  *       *
   *     *
     ***
"#,

r#"
      *****
   **       **
 **           **
**             **
 **           **
   **       **
      *****
"#,

r#"
        *********
    ***         ***
  **               **
 **                 **
 **                 **
  **               **
    ***         ***
        *********
"#

];

    for frame in frames {

        clear();

        center("NEXORA");

        println!();

        println!("{}", frame);

        thread::sleep(Duration::from_millis(FRAME_DELAY));

    }

}

/// ======================================================
/// STEP 4
/// Butterfly Appears
/// ======================================================
fn butterfly() {

    clear();

    center("NEXORA");

    println!();

    println!(r#"

              .==-.                   .-==.
               \()8`-._  `.   .'  _.-'8()/
               (88"   ::.  \./  .::   "88)
                \_.'`-::::.(#).::::-'`._/
                  `._... .q(_)p. ..._.'
                    ""-..-'|=|`-..-""
                    .""' .'|=|`. `"".
                  ,':8(o)./|=|\.(o)8:`.
                 (O :8 ::/ \_/ \:: 8: O)
                  \O `::/       \::' O/
                   ""--'         `--""

"#);

    thread::sleep(Duration::from_secs(2));

}

/// ======================================================
/// STEP 5
/// Final Splash
/// ======================================================
fn tagline() {

    clear();

    center("NEXORA OS");
    center("");
    center("UNLEASH • EVOLVE • EXCEL");

    thread::sleep(Duration::from_secs(2));

}

/// ======================================================
/// STEP 6
/// Loading...
/// ======================================================
fn loading() {

    for i in 0..=100 {

        clear();

        center("NEXORA OS");

        println!();

        println!("Loading... {}%", i);

        thread::sleep(Duration::from_millis(25));

    }

}

/// Utility
fn clear() {

    print!("\x1B[2J\x1B[1;1H");

}

/// Utility
fn center(text: &str) {

    println!("{:^80}", text);

}
