fn main() {
    let color = skycolor::get_sky_color();
    draw::set_color(color);
    print::set_color(color);
}
