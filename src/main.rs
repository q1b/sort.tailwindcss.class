use std::time::Instant;
/*
    align-baseline
    animate
    aspect
    appearance
    absolute
    accent

    break -{ break }
    blur
    brightness
    break-after
    break-before
    basic
    backdrop
    bottom
    bg
    block
    border
    box-border
    box-content
    box-decoration

    caret
    clear
    content-none
    contrast
    cursor

    duration
    delay
    decoration
    divide
    drop-shadow

    ease

    float
    flow-root
    from
    fill
    flex


    grow
    grid
    grayscale

    hue
    h

    isolate
    indent
    inset
    italic
    invert
    inline
    inline-flex
    invisible
    inline-block

    justify

    k*

    left
    list
    lining *font-variants
    leading

    min
    mix
    max
    m

    no-underline *text-decoration
    not-sr-only *screen-readers
    normal *font-variants
    normal-case *text-transform

    opacity
    order
    outline
    overflow
    object
    origin

    p
    pointer
    place

    *q

    right
    resize
    ring

    scroll
    shadow
    sr-only
    snap
    sticky
    static
    scale
    skew
    select
    stroke
    space
    sepia
    self
    saturate

    translate
    touch
    transition
    truncate
    tracking
    top
    to
    text

    underline

    visible

    whitespace
    will-change


*/

/* Basic prefix in tailwindcss
        - positioning
            relative absolute sticky fixed
            top,right,bottom,left,inset
        - w-A
        - h-A
        - p-A
        - m-A
        - max-[w,h]-A ,min-[w,h]-A
        - shadow-A
        - ring-A
        - outline-A
        - list-A
        - border-A
        - font-A
        - bg-A
        - text-A
        - overflow-A
        - scroll-A
        - backdrop-A
        - animate-A

        - transition
            duration
            delay
            ease

        - transform
            scale
            rotate
            skew
            translate-[x,y]

*/


fn main() {
	// This is my class
	// const INITIAL_CLASS: &str = "flex absolute items-center top-0 left-0 right-9 bottom-10 justify-content m-4 p-4 px-2 border border-blue-500 shadow-xl shadow-blue-700 top-0 shrink-0 grow opacity-90 text-opacity-90 bg-opacity-50 bg-contain place-content-between bg-gradient-to-tr from-blue-500 to-sky-600 transition-opacity text-xl text-blue-900 no-underline font-bold italic translate-x-20 scale-100 rotate-90 duration-900 delay-50 ease-in-out animate-pulse not-sr-only select-none";
    let a = "shadow-xl underline underline-offset-1 shadow-blue-400 bg-cover place-content-between focus:text-bold hover:text-white grid items-center justify-center hover:bg-white bg-white/90 text-xl transition-opacity duration-1000 delay-500 ease-in-out scale-200 ";
    let start = Instant::now();
	let result= tailwind_cli::sort_tailwindcss_classes(a);
    let duration = start.elapsed();
	println!("The greatr result {}",result);
    println!("Dur {:?}",duration);
}
