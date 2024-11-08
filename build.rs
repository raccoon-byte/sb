fn main() {
    println!("cargo:rustc-flags=-L /usr/X11R6/lib -l X11 -l Xft -l Xinerama");
}
