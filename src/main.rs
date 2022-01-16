// use std::vec;
use std::time::{Instant};
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

#[derive(Debug)]
struct TailwindcssClass {
    value: String,
    priority: u8,
}

fn main() {
    const INITIAL_CLASS: &str = " flex select-none items-center shrink-0 justify-center place-content-between text-xl text-blue-600 pt-10 py-4 px-3  p-0 p-1 p-2 p-3 p-4 p-5 p-6 p-7 p-8 p-9 p-[1px] p-[3vh] p-[3.555em] pt-0 pt-1 pt-2 pt-3 pt-4 pt-5 pt-6 pt-7 pt-8 pt-9 pt-[1px] pt-[3vh] pt-[3.555em] m-0 m-1 m-2 m-3 m-4 m-5 m-6 m-7 m-8 m-9 m-[1px] m-[3vh] m-[3.555em] mx-0 mx-1 mx-2 mx-3 mx-4 mx-5 mx-6 mx-7 mx-8 mx-9 mx-[1px] mx-[3vh] mx-[3.555em] top-0 top-1 top-2 top-3 top-4 top-5 top-6 top-7 top-8 top-9 top-[1px] top-[3vh] top-[3.555em] text-red-200 text-red-400 text-orange-200 text-orange-400 text-blue-200 text-blue-400 text-red-200/10 text-red-200/20 text-red-400/10 text-red-400/20 text-orange-200/10 text-orange-200/20 text-orange-400/10 text-orange-400/20 text-blue-200/10 text-blue-200/20 text-blue-400/10 text-blue-400/20 text-[#525343] text-[#124453] text-[#942] bg-red-200 bg-red-400 bg-orange-200 bg-orange-400 bg-blue-200 bg-blue-400 bg-red-200/10 bg-red-200/20 bg-red-400/10 bg-red-400/20 bg-orange-200/10 bg-orange-200/20 bg-orange-400/10 bg-orange-400/20 bg-blue-200/10 bg-blue-200/20 bg-blue-400/10 bg-blue-400/20 bg-[#525343] bg-[#124453] bg-[#942] border-red-200 border-red-400 border-orange-200 border-orange-400 border-blue-200 border-blue-400 border-red-200/10 border-red-200/20 border-red-400/10 border-red-400/20 border-orange-200/10 border-orange-200/20 border-orange-400/10 border-orange-400/20 border-blue-200/10 border-blue-200/20 border-blue-400/10 border-blue-400/20 border-[#525343] border-[#124453] border-[#942] text-sm text-lg text-xl rounded-sm rounded-lg rounded-xl grid-cols-1 grid-cols-2 grid-cols-[1fr,3em] grid-cols-[20px,min-content,1fr] scale-100 rotate-45 translate-x-10 transition-opacity duration-900 delay-100 ";
    let mut init_itra = INITIAL_CLASS.chars();

    let mut box_model: Vec<TailwindcssClass> = Vec::new();

    let mut positioning: Vec<TailwindcssClass> = Vec::new();

    let mut alignment: Vec<TailwindcssClass> = Vec::new();

    let mut flex_grid: Vec<TailwindcssClass> = Vec::new();

    let mut typography: Vec<TailwindcssClass> = Vec::new();
    // Visuals :- Classes like bg, shadows, backdrop-shadows
    let mut visuals: Vec<TailwindcssClass> = Vec::new();
    // Effects brightness blur filters as well mix
    let mut effects: Vec<TailwindcssClass> = Vec::new();
    // Transform
    let mut transforms: Vec<TailwindcssClass> = Vec::new();
    // ANimation & Transition
    let mut motion: Vec<TailwindcssClass> = Vec::new();
    // Misc will contain classes like caret color or scroll snap classes;
    let mut misc: Vec<TailwindcssClass> = Vec::new();

	let start = Instant::now();
	
    loop {
        match init_itra.next() {
            Some(v) => {
                match v {
					'a' => { 
    						let d1 = init_itra.next().unwrap(); 
    						match d1 {
							'b' => {
    								//'absolute'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								positioning.push(TailwindcssClass {
    								  value: v4,
    								  priority: 1,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							's' => {
    								//'aspect'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								box_model.push(TailwindcssClass {
    								  value: v4,
    								  priority: 0,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							'u' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									't' => { 
    										let d3 = init_itra.next().unwrap(); 
    										match d3 {
											'o' => { 
    												let d4 = init_itra.next().unwrap(); 
    												match d4 {
													'-' => { 
    														let d5 = init_itra.next().unwrap(); 
    														match d5 {
															'c' => {
    																//'auto-cols'
    																let mut v2 = init_itra.next().unwrap();
    																let mut v3 = String::new();
    																// println!("Scale Valie{}", v2);
    																while v2 != ' ' {
    																    v3 = v3 + &String::from(v2);
    																    match init_itra.next() {
    																        Some(next_char_value) => v2 = next_char_value,
    																        None => break,
    																    }
    																}
    																let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string() + &v3; 
    																flex_grid.push(TailwindcssClass {
    																  value: v4,
    																  priority: 6,
    																});
    																match v2 {
																	_ => println!("break at d6"),
																}
															}
															'r' => {
    																//'auto-rows'
    																let mut v2 = init_itra.next().unwrap();
    																let mut v3 = String::new();
    																// println!("Scale Valie{}", v2);
    																while v2 != ' ' {
    																    v3 = v3 + &String::from(v2);
    																    match init_itra.next() {
    																        Some(next_char_value) => v2 = next_char_value,
    																        None => break,
    																    }
    																}
    																let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string() + &v3; 
    																flex_grid.push(TailwindcssClass {
    																  value: v4,
    																  priority: 7,
    																});
    																match v2 {
																	_ => println!("break at d6"),
																}
															}
															_ => println!("break at d5"),
														}
													}
													_ => println!("break at d4"),
												}
											}
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							'n' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									't' => {
    										//'antialiased'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										typography.push(TailwindcssClass {
    										  value: v4,
    										  priority: 4,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									'i' => {
    										//'animate'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										motion.push(TailwindcssClass {
    										  value: v4,
    										  priority: 1,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							'l' => {
    								//'align'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								typography.push(TailwindcssClass {
    								  value: v4,
    								  priority: 14,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							'c' => {
    								//'accent'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								misc.push(TailwindcssClass {
    								  value: v4,
    								  priority: 2,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							'p' => {
    								//'appearance'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								misc.push(TailwindcssClass {
    								  value: v4,
    								  priority: 2,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							_ => println!("break at d1"),
						}
					}
					'b' => { 
    						let d1 = init_itra.next().unwrap(); 
    						match d1 {
							'o' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									't' => {
    										//'bottom'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										positioning.push(TailwindcssClass {
    										  value: v4,
    										  priority: 5,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									'r' => { 
    										let d3 = init_itra.next().unwrap(); 
    										match d3 {
											'd' => { 
    												let d4 = init_itra.next().unwrap(); 
    												match d4 {
													'e' => { 
    														let d5 = init_itra.next().unwrap(); 
    														match d5 {
															'r' => { 
    																let d6 = init_itra.next().unwrap(); 
    																match d6 {
																	'-' => { 
    																		let d7 = init_itra.next().unwrap(); 
    																		match d7 {
                                                                                'x' => {
    																				//'border-x'
    																				let mut v2 = init_itra.next().unwrap();
    																				let mut v3 = String::new();
    																				// println!("Scale Valie{}", v2);
    																				while v2 != ' ' {
    																				    v3 = v3 + &String::from(v2);
    																				    match init_itra.next() {
    																				        Some(next_char_value) => v2 = next_char_value,
    																				        None => break,
    																				    }
    																				}
    																				let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3; 
    																				box_model.push(TailwindcssClass {
    																				  value: v4,
    																				  priority: 33,
    																				});
    																				match v2 {
																					_ => println!("break at d8"),
																				}
																			}
																			'y' => {
    																				//'border-y'
    																				let mut v2 = init_itra.next().unwrap();
    																				let mut v3 = String::new();
    																				// println!("Scale Valie{}", v2);
    																				while v2 != ' ' {
    																				    v3 = v3 + &String::from(v2);
    																				    match init_itra.next() {
    																				        Some(next_char_value) => v2 = next_char_value,
    																				        None => break,
    																				    }
    																				}
    																				let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3; 
    																				box_model.push(TailwindcssClass {
    																				  value: v4,
    																				  priority: 34,
    																				});
    																				match v2 {
																					_ => println!("break at d8"),
																				}
																			}
																			't' => {
    																				//'border-t'
    																				let mut v2 = init_itra.next().unwrap();
    																				let mut v3 = String::new();
    																				// println!("Scale Valie{}", v2);
    																				while v2 != ' ' {
    																				    v3 = v3 + &String::from(v2);
    																				    match init_itra.next() {
    																				        Some(next_char_value) => v2 = next_char_value,
    																				        None => break,
    																				    }
    																				}
    																				let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3; 
    																				box_model.push(TailwindcssClass {
    																				  value: v4,
    																				  priority: 35,
    																				});
    																				match v2 {
																					_ => println!("break at d8"),
																				}
																			}
																			'l' => {
    																				//'border-l'
    																				let mut v2 = init_itra.next().unwrap();
    																				let mut v3 = String::new();
    																				// println!("Scale Valie{}", v2);
    																				while v2 != ' ' {
    																				    v3 = v3 + &String::from(v2);
    																				    match init_itra.next() {
    																				        Some(next_char_value) => v2 = next_char_value,
    																				        None => break,
    																				    }
    																				}
    																				let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3; 
    																				box_model.push(TailwindcssClass {
    																				  value: v4,
    																				  priority: 36,
    																				});
    																				match v2 {
																					_ => println!("break at d8"),
																				}
																			}
																			'r' => {
    																				//'border-r'
    																				let mut v2 = init_itra.next().unwrap();
    																				let mut v3 = String::new();
    																				// println!("Scale Valie{}", v2);
    																				while v2 != ' ' {
    																				    v3 = v3 + &String::from(v2);
    																				    match init_itra.next() {
    																				        Some(next_char_value) => v2 = next_char_value,
    																				        None => break,
    																				    }
    																				}
    																				let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3; 
    																				box_model.push(TailwindcssClass {
    																				  value: v4,
    																				  priority: 37,
    																				});
    																				match v2 {
																					_ => println!("break at d8"),
																				}
																			}
																			'b' => {
    																				//'border-b'
    																				let mut v2 = init_itra.next().unwrap();
    																				let mut v3 = String::new();
    																				// println!("Scale Valie{}", v2);
    																				while v2 != ' ' {
    																				    v3 = v3 + &String::from(v2);
    																				    match init_itra.next() {
    																				        Some(next_char_value) => v2 = next_char_value,
    																				        None => break,
    																				    }
    																				}
    																				let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3; 
    																				box_model.push(TailwindcssClass {
    																				  value: v4,
    																				  priority: 38,
    																				});
    																				match v2 {
																					_ => println!("break at d8"),
																				}
																			}
																			_ => {
    																				// border-*
    																				let mut v2 = init_itra.next().unwrap();
    																				let mut v3 = String::new();
    																				// println!("Scale Valie{}", v2);
    																				while v2 != ' ' {
    																				    v3 = v3 + &String::from(v2);
    																				    match init_itra.next() {
    																				        Some(next_char_value) => v2 = next_char_value,
    																				        None => break,
    																				    }
    																				}
    																				let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3; 
    																				box_model.push(TailwindcssClass {
    																				  value: v4,
    																				  priority: 31,
    																				});
    																				match v2 {
																					_ => println!("break at d8"),
																				}
																			}
																		}
																	}
																	_ => println!("break at d6"),
																}
															}
															_ => println!("break at d5"),
														}
													}
													_ => println!("break at d4"),
												}
											}
											_ => println!("break at d3"),
										}
									}
									'x' => { 
    										let d3 = init_itra.next().unwrap(); 
    										match d3 {
											'-' => { 
    												let d4 = init_itra.next().unwrap(); 
    												match d4 {
													'd' => {
    														//'box-decoration'
    														let mut v2 = init_itra.next().unwrap();
    														let mut v3 = String::new();
    														// println!("Scale Valie{}", v2);
    														while v2 != ' ' {
    														    v3 = v3 + &String::from(v2);
    														    match init_itra.next() {
    														        Some(next_char_value) => v2 = next_char_value,
    														        None => break,
    														    }
    														}
    														let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string() + &v3; 
    														box_model.push(TailwindcssClass {
    														  value: v4,
    														  priority: 74,
    														});
    														match v2 {
															_ => println!("break at d5"),
														}
													}
													'b' => {
    														//'box-border'
    														let mut v2 = init_itra.next().unwrap();
    														let mut v3 = String::new();
    														// println!("Scale Valie{}", v2);
    														while v2 != ' ' {
    														    v3 = v3 + &String::from(v2);
    														    match init_itra.next() {
    														        Some(next_char_value) => v2 = next_char_value,
    														        None => break,
    														    }
    														}
    														let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string() + &v3; 
    														box_model.push(TailwindcssClass {
    														  value: v4,
    														  priority: 75,
    														});
    														match v2 {
															_ => println!("break at d5"),
														}
													}
													'c' => {
    														//'box-content'
    														let mut v2 = init_itra.next().unwrap();
    														let mut v3 = String::new();
    														// println!("Scale Valie{}", v2);
    														while v2 != ' ' {
    														    v3 = v3 + &String::from(v2);
    														    match init_itra.next() {
    														        Some(next_char_value) => v2 = next_char_value,
    														        None => break,
    														    }
    														}
    														let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string() + &v3; 
    														box_model.push(TailwindcssClass {
    														  value: v4,
    														  priority: 76,
    														});
    														match v2 {
															_ => println!("break at d5"),
														}
													}
													_ => println!("break at d4"),
												}
											}
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							'l' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									'o' => {
    										//'block'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										box_model.push(TailwindcssClass {
    										  value: v4,
    										  priority: 1,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									'u' => {
    										//'blur'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										effects.push(TailwindcssClass {
    										  value: v4,
    										  priority: 3,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							'r' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									'e' => { 
    										let d3 = init_itra.next().unwrap(); 
    										match d3 {
											'a' => { 
    												let d4 = init_itra.next().unwrap(); 
    												match d4 {
													'k' => { 
    														let d5 = init_itra.next().unwrap(); 
    														match d5 {
															'-' => { 
    																let d6 = init_itra.next().unwrap(); 
    																match d6 {
																	'a' => { 
    																		let d7 = init_itra.next().unwrap(); 
    																		match d7 {
																			'f' => {
    																				//'break-after'
    																				let mut v2 = init_itra.next().unwrap();
    																				let mut v3 = String::new();
    																				// println!("Scale Valie{}", v2);
    																				while v2 != ' ' {
    																				    v3 = v3 + &String::from(v2);
    																				    match init_itra.next() {
    																				        Some(next_char_value) => v2 = next_char_value,
    																				        None => break,
    																				    }
    																				}
    																				let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3; 
    																				box_model.push(TailwindcssClass {
    																				  value: v4,
    																				  priority: 71,
    																				});
    																				match v2 {
																					_ => println!("break at d8"),
																				}
																			}
																			'l' => {
    																				//'break-all'
    																				let mut v2 = init_itra.next().unwrap();
    																				let mut v3 = String::new();
    																				// println!("Scale Valie{}", v2);
    																				while v2 != ' ' {
    																				    v3 = v3 + &String::from(v2);
    																				    match init_itra.next() {
    																				        Some(next_char_value) => v2 = next_char_value,
    																				        None => break,
    																				    }
    																				}
    																				let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3; 
    																				typography.push(TailwindcssClass {
    																				  value: v4,
    																				  priority: 16,
    																				});
    																				match v2 {
																					_ => println!("break at d8"),
																				}
																			}
																			_ => println!("break at d7"),
																		}
																	}
																	'b' => {
    																		//'break-before'
    																		let mut v2 = init_itra.next().unwrap();
    																		let mut v3 = String::new();
    																		// println!("Scale Valie{}", v2);
    																		while v2 != ' ' {
    																		    v3 = v3 + &String::from(v2);
    																		    match init_itra.next() {
    																		        Some(next_char_value) => v2 = next_char_value,
    																		        None => break,
    																		    }
    																		}
    																		let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3; 
    																		box_model.push(TailwindcssClass {
    																		  value: v4,
    																		  priority: 72,
    																		});
    																		match v2 {
																			_ => println!("break at d7"),
																		}
																	}
																	'i' => {
    																		//'break-inside'
    																		let mut v2 = init_itra.next().unwrap();
    																		let mut v3 = String::new();
    																		// println!("Scale Valie{}", v2);
    																		while v2 != ' ' {
    																		    v3 = v3 + &String::from(v2);
    																		    match init_itra.next() {
    																		        Some(next_char_value) => v2 = next_char_value,
    																		        None => break,
    																		    }
    																		}
    																		let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3; 
    																		box_model.push(TailwindcssClass {
    																		  value: v4,
    																		  priority: 73,
    																		});
    																		match v2 {
																			_ => println!("break at d7"),
																		}
																	}
																	'n' => {
    																		//'break-normal'
    																		let mut v2 = init_itra.next().unwrap();
    																		let mut v3 = String::new();
    																		// println!("Scale Valie{}", v2);
    																		while v2 != ' ' {
    																		    v3 = v3 + &String::from(v2);
    																		    match init_itra.next() {
    																		        Some(next_char_value) => v2 = next_char_value,
    																		        None => break,
    																		    }
    																		}
    																		let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3; 
    																		typography.push(TailwindcssClass {
    																		  value: v4,
    																		  priority: 16,
    																		});
    																		match v2 {
																			_ => println!("break at d7"),
																		}
																	}
																	'w' => {
    																		//'break-words'
    																		let mut v2 = init_itra.next().unwrap();
    																		let mut v3 = String::new();
    																		// println!("Scale Valie{}", v2);
    																		while v2 != ' ' {
    																		    v3 = v3 + &String::from(v2);
    																		    match init_itra.next() {
    																		        Some(next_char_value) => v2 = next_char_value,
    																		        None => break,
    																		    }
    																		}
    																		let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3; 
    																		typography.push(TailwindcssClass {
    																		  value: v4,
    																		  priority: 16,
    																		});
    																		match v2 {
																			_ => println!("break at d7"),
																		}
																	}
																	_ => println!("break at d6"),
																}
															}
															_ => println!("break at d5"),
														}
													}
													_ => println!("break at d4"),
												}
											}
											_ => println!("break at d3"),
										}
									}
									'i' => {
    										//'brightness'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										effects.push(TailwindcssClass {
    										  value: v4,
    										  priority: 4,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							'a' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									's' => {
    										//'basis'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										flex_grid.push(TailwindcssClass {
    										  value: v4,
    										  priority: 2,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									'c' => {
    										//'backdrop'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										effects.push(TailwindcssClass {
    										  value: v4,
    										  priority: 11,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							'g' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									'-' => { 
    										let d3 = init_itra.next().unwrap(); 
    										match d3 {
											'g' => {
    												//'bg-gradient-to-'
    												let mut v2 = init_itra.next().unwrap();
    												let mut v3 = String::new();
    												// println!("Scale Valie{}", v2);
    												while v2 != ' ' {
    												    v3 = v3 + &String::from(v2);
    												    match init_itra.next() {
    												        Some(next_char_value) => v2 = next_char_value,
    												        None => break,
    												    }
    												}
    												let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string() + &v3; 
    												visuals.push(TailwindcssClass {
    												  value: v4,
    												  priority: 11,
    												});
    												match v2 {
													_ => println!("break at d4"),
												}
											}
											'n' => {
    												//'bg-none'
    												let mut v2 = init_itra.next().unwrap();
    												let mut v3 = String::new();
    												// println!("Scale Valie{}", v2);
    												while v2 != ' ' {
    												    v3 = v3 + &String::from(v2);
    												    match init_itra.next() {
    												        Some(next_char_value) => v2 = next_char_value,
    												        None => break,
    												    }
    												}
    												let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string() + &v3; 
    												visuals.push(TailwindcssClass {
    												  value: v4,
    												  priority: 11,
    												});
    												match v2 {
													_ => println!("break at d4"),
												}
											}
											'b' => {
    												//'bg-blend'
    												let mut v2 = init_itra.next().unwrap();
    												let mut v3 = String::new();
    												// println!("Scale Valie{}", v2);
    												while v2 != ' ' {
    												    v3 = v3 + &String::from(v2);
    												    match init_itra.next() {
    												        Some(next_char_value) => v2 = next_char_value,
    												        None => break,
    												    }
    												}
    												let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string() + &v3; 
    												effects.push(TailwindcssClass {
    												  value: v4,
    												  priority: 3,
    												});
    												match v2 {
													_ => println!("break at d4"),
												}
											}
											_ => {
                                                // bg-*
    												let mut v2 = init_itra.next().unwrap();
    												let mut v3 = String::new();
    												// println!("Scale Valie{}", v2);
    												while v2 != ' ' {
    												    v3 = v3 + &String::from(v2);
    												    match init_itra.next() {
    												        Some(next_char_value) => v2 = next_char_value,
    												        None => break,
    												    }
    												}
    												let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string() + &v3; 
    												visuals.push(TailwindcssClass {
    												  value: v4,
    												  priority: 1,
    												});
    												match v2 {
												    	_ => println!("break at d4"),
												    }
                                            }
										}
									}
									_ => println!("break at d2"),
								}
							}
							_ => println!("break at d1"),
						}
					}
					'c' => { 
    						let d1 = init_itra.next().unwrap(); 
    						match d1 {
							'o' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									'n' => { 
    										let d3 = init_itra.next().unwrap(); 
    										match d3 {
											't' => { 
    												let d4 = init_itra.next().unwrap(); 
    												match d4 {
													'e' => { 
    														let d5 = init_itra.next().unwrap(); 
    														match d5 {
															'n' => { 
    																let d6 = init_itra.next().unwrap(); 
    																match d6 {
																	't' => { 
    																		let d7 = init_itra.next().unwrap(); 
    																		match d7 {
																			's' => {
    																				//'contents'
    																				let mut v2 = init_itra.next().unwrap();
    																				let mut v3 = String::new();
    																				// println!("Scale Valie{}", v2);
    																				while v2 != ' ' {
    																				    v3 = v3 + &String::from(v2);
    																				    match init_itra.next() {
    																				        Some(next_char_value) => v2 = next_char_value,
    																				        None => break,
    																				    }
    																				}
    																				let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3; 
    																				box_model.push(TailwindcssClass {
    																				  value: v4,
    																				  priority: 1,
    																				});
    																				match v2 {
																					_ => println!("break at d8"),
																				}
																			}
																			'-' => {
    																				//'content-none'
    																				let mut v2 = init_itra.next().unwrap();
    																				let mut v3 = String::new();
    																				// println!("Scale Valie{}", v2);
    																				while v2 != ' ' {
    																				    v3 = v3 + &String::from(v2);
    																				    match init_itra.next() {
    																				        Some(next_char_value) => v2 = next_char_value,
    																				        None => break,
    																				    }
    																				}
    																				let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3; 
    																				typography.push(TailwindcssClass {
    																				  value: v4,
    																				  priority: 16,
    																				});
    																				match v2 {
																					_ => println!("break at d8"),
																				}
																			}
																			_ => {
                                                                                    																				//'content'
    																				let mut v2 = init_itra.next().unwrap();
    																				let mut v3 = String::new();
    																				// println!("Scale Valie{}", v2);
    																				while v2 != ' ' {
    																				    v3 = v3 + &String::from(v2);
    																				    match init_itra.next() {
    																				        Some(next_char_value) => v2 = next_char_value,
    																				        None => break,
    																				    }
    																				}
    																				let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3; 
    																				alignment.push(TailwindcssClass {
    																				  value: v4,
    																				  priority: 8,
    																				});
    																				match v2 {
																					_ => println!("break at d8"),
																				}
                                                                            },
																		}
																	}
																	_ => println!("break at d6"),
																}
															}
															_ => println!("break at d5"),
														}
													}
													'a' => {
    														//'container'
    														let mut v2 = init_itra.next().unwrap();
    														let mut v3 = String::new();
    														// println!("Scale Valie{}", v2);
    														while v2 != ' ' {
    														    v3 = v3 + &String::from(v2);
    														    match init_itra.next() {
    														        Some(next_char_value) => v2 = next_char_value,
    														        None => break,
    														    }
    														}
    														let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string() + &v3; 
    														box_model.push(TailwindcssClass {
    														  value: v4,
    														  priority: 2,
    														});
    														match v2 {
															_ => println!("break at d5"),
														}
													}
													'r' => {
    														//'contrast'
    														let mut v2 = init_itra.next().unwrap();
    														let mut v3 = String::new();
    														// println!("Scale Valie{}", v2);
    														while v2 != ' ' {
    														    v3 = v3 + &String::from(v2);
    														    match init_itra.next() {
    														        Some(next_char_value) => v2 = next_char_value,
    														        None => break,
    														    }
    														}
    														let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string() + &v3; 
    														effects.push(TailwindcssClass {
    														  value: v4,
    														  priority: 4,
    														});
    														match v2 {
															_ => println!("break at d5"),
														}
													}
													_ => println!("break at d4"),
												}
											}
											_ => println!("break at d3"),
										}
									}
									'l' => { 
    										let d3 = init_itra.next().unwrap(); 
    										match d3 {
											'u' => {
    												//'columns'
    												let mut v2 = init_itra.next().unwrap();
    												let mut v3 = String::new();
    												// println!("Scale Valie{}", v2);
    												while v2 != ' ' {
    												    v3 = v3 + &String::from(v2);
    												    match init_itra.next() {
    												        Some(next_char_value) => v2 = next_char_value,
    												        None => break,
    												    }
    												}
    												let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string() + &v3; 
    												box_model.push(TailwindcssClass {
    												  value: v4,
    												  priority: 3,
    												});
    												match v2 {
													_ => println!("break at d4"),
												}
											}
											's' => {
    												//'cols'
    												let mut v2 = init_itra.next().unwrap();
    												let mut v3 = String::new();
    												// println!("Scale Valie{}", v2);
    												while v2 != ' ' {
    												    v3 = v3 + &String::from(v2);
    												    match init_itra.next() {
    												        Some(next_char_value) => v2 = next_char_value,
    												        None => break,
    												    }
    												}
    												let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string() + &v3; 
    												flex_grid.push(TailwindcssClass {
    												  value: v4,
    												  priority: 2,
    												});
    												match v2 {
													_ => println!("break at d4"),
												}
											}
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							'l' => {
    								//'clear'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								box_model.push(TailwindcssClass {
    								  value: v4,
    								  priority: 10,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							'a' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									'p' => {
    										//'capitalize'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										typography.push(TailwindcssClass {
    										  value: v4,
    										  priority: 11,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									'r' => {
    										//'caret'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										misc.push(TailwindcssClass {
    										  value: v4,
    										  priority: 1,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							'u' => {
    								//'cursor'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								misc.push(TailwindcssClass {
    								  value: v4,
    								  priority: 1,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							_ => println!("break at d1"),
						}
					}
					'd' => { 
    						let d1 = init_itra.next().unwrap(); 
    						match d1 {
							'i' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									'v' => {
    										//'divide'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										box_model.push(TailwindcssClass {
    										  value: v4,
    										  priority: 21,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									'a' => {
    										//'diagonal-fractions'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										typography.push(TailwindcssClass {
    										  value: v4,
    										  priority: 9,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							'e' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									'c' => {
    										//'decoration'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										typography.push(TailwindcssClass {
    										  value: v4,
    										  priority: 10,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									'l' => {
    										//'delay'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										motion.push(TailwindcssClass {
    										  value: v4,
    										  priority: 4,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							'r' => {
    								//'drop-shadow'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								effects.push(TailwindcssClass {
    								  value: v4,
    								  priority: 5,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							'u' => {
    								//'duration'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								motion.push(TailwindcssClass {
    								  value: v4,
    								  priority: 3,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							_ => println!("break at d1"),
						}
					}
					'e' => { 
    						let d1 = init_itra.next().unwrap(); 
    						match d1 {
							'a' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									's' => { 
    										let d3 = init_itra.next().unwrap(); 
    										match d3 {
											'e' => { 
    												let d4 = init_itra.next().unwrap(); 
    												match d4 {
													'-' => { 
    														let d5 = init_itra.next().unwrap(); 
    														match d5 {
															'l' => {
    																//'ease-linear'
    																let mut v2 = init_itra.next().unwrap();
    																let mut v3 = String::new();
    																// println!("Scale Valie{}", v2);
    																while v2 != ' ' {
    																    v3 = v3 + &String::from(v2);
    																    match init_itra.next() {
    																        Some(next_char_value) => v2 = next_char_value,
    																        None => break,
    																    }
    																}
    																let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string() + &v3; 
    																motion.push(TailwindcssClass {
    																  value: v4,
    																  priority: 5,
    																});
    																match v2 {
																	_ => println!("break at d6"),
																}
															}
															'i' => {
    																//'ease-in'
    																let mut v2 = init_itra.next().unwrap();
    																let mut v3 = String::new();
    																// println!("Scale Valie{}", v2);
    																while v2 != ' ' {
    																    v3 = v3 + &String::from(v2);
    																    match init_itra.next() {
    																        Some(next_char_value) => v2 = next_char_value,
    																        None => break,
    																    }
    																}
    																let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string() + &v3; 
    																motion.push(TailwindcssClass {
    																  value: v4,
    																  priority: 5,
    																});
    																match v2 {
																	_ => println!("break at d6"),
																}
															}
															'o' => {
    																//'ease-out'
    																let mut v2 = init_itra.next().unwrap();
    																let mut v3 = String::new();
    																// println!("Scale Valie{}", v2);
    																while v2 != ' ' {
    																    v3 = v3 + &String::from(v2);
    																    match init_itra.next() {
    																        Some(next_char_value) => v2 = next_char_value,
    																        None => break,
    																    }
    																}
    																let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string() + &v3; 
    																motion.push(TailwindcssClass {
    																  value: v4,
    																  priority: 5,
    																});
    																match v2 {
																	_ => println!("break at d6"),
																}
															}
															_ => println!("break at d5"),
														}
													}
													_ => println!("break at d4"),
												}
											}
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							_ => println!("break at d1"),
						}
					}
					'f' => { 
    						let d1 = init_itra.next().unwrap(); 
    						match d1 {
							'i' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									'x' => {
    										//'fixed'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										positioning.push(TailwindcssClass {
    										  value: v4,
    										  priority: 1,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									'l' => {
    										//'fill'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										visuals.push(TailwindcssClass {
    										  value: v4,
    										  priority: 1,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							'l' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									'e' => {
    										//'flex'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										box_model.push(TailwindcssClass {
    										  value: v4,
    										  priority: 1,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									'o' => { 
    										let d3 = init_itra.next().unwrap(); 
    										match d3 {
											'w' => {
    												//'flow-root'
    												let mut v2 = init_itra.next().unwrap();
    												let mut v3 = String::new();
    												// println!("Scale Valie{}", v2);
    												while v2 != ' ' {
    												    v3 = v3 + &String::from(v2);
    												    match init_itra.next() {
    												        Some(next_char_value) => v2 = next_char_value,
    												        None => break,
    												    }
    												}
    												let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string() + &v3; 
    												box_model.push(TailwindcssClass {
    												  value: v4,
    												  priority: 1,
    												});
    												match v2 {
													_ => println!("break at d4"),
												}
											}
											'a' => {
    												//'float'
    												let mut v2 = init_itra.next().unwrap();
    												let mut v3 = String::new();
    												// println!("Scale Valie{}", v2);
    												while v2 != ' ' {
    												    v3 = v3 + &String::from(v2);
    												    match init_itra.next() {
    												        Some(next_char_value) => v2 = next_char_value,
    												        None => break,
    												    }
    												}
    												let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string() + &v3; 
    												box_model.push(TailwindcssClass {
    												  value: v4,
    												  priority: 10,
    												});
    												match v2 {
													_ => println!("break at d4"),
												}
											}
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							'o' => {
    								//'font-'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								typography.push(TailwindcssClass {
    								  value: v4,
    								  priority: 1,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							'r' => {
    								//'from-'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								visuals.push(TailwindcssClass {
    								  value: v4,
    								  priority: 12,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							_ => println!("break at d1"),
						}
					}
					'g' => { 
    						let d1 = init_itra.next().unwrap(); 
    						match d1 {
							'r' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									'i' => { 
    										let d3 = init_itra.next().unwrap(); 
    										match d3 {
											'd' => { 
    												let d4 = init_itra.next().unwrap(); 
    												match d4 {
													'-' => { 
    														let d5 = init_itra.next().unwrap(); 
    														match d5 {
															'r' => {
    																//'grid-rows'
    																let mut v2 = init_itra.next().unwrap();
    																let mut v3 = String::new();
    																// println!("Scale Valie{}", v2);
    																while v2 != ' ' {
    																    v3 = v3 + &String::from(v2);
    																    match init_itra.next() {
    																        Some(next_char_value) => v2 = next_char_value,
    																        None => break,
    																    }
    																}
    																let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string() + &v3; 
    																flex_grid.push(TailwindcssClass {
    																  value: v4,
    																  priority: 3,
    																});
    																match v2 {
																	_ => println!("break at d6"),
																}
															}
															'c' => {
    																//'grid-cols'
    																let mut v2 = init_itra.next().unwrap();
    																let mut v3 = String::new();
    																// println!("Scale Valie{}", v2);
    																while v2 != ' ' {
    																    v3 = v3 + &String::from(v2);
    																    match init_itra.next() {
    																        Some(next_char_value) => v2 = next_char_value,
    																        None => break,
    																    }
    																}
    																let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string() + &v3; 
    																flex_grid.push(TailwindcssClass {
    																  value: v4,
    																  priority: 4,
    																});
    																match v2 {
																	_ => println!("break at d6"),
																}
															}
															'f' => {
    																//'grid-flow'
    																let mut v2 = init_itra.next().unwrap();
    																let mut v3 = String::new();
    																// println!("Scale Valie{}", v2);
    																while v2 != ' ' {
    																    v3 = v3 + &String::from(v2);
    																    match init_itra.next() {
    																        Some(next_char_value) => v2 = next_char_value,
    																        None => break,
    																    }
    																}
    																let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string() + &v3; 
    																flex_grid.push(TailwindcssClass {
    																  value: v4,
    																  priority: 5,
    																});
    																match v2 {
																	_ => println!("break at d6"),
																}
															}
															_ => println!("break at d5"),
														}
													}
                                                    _ => {
                                                            //'grid'
                                                            let mut v2 = init_itra.next().unwrap();
                                                            let mut v3 = String::new();
                                                            // println!("Scale Valie{}", v2);
                                                            while v2 != ' ' {
                                                                v3 = v3 + &String::from(v2);
                                                                match init_itra.next() {
                                                                    Some(next_char_value) => v2 = next_char_value,
                                                                    None => break,
                                                                }
                                                            }
                                                            let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string() + &v3; 
                                                            box_model.push(TailwindcssClass {
                                                              value: v4,
                                                              priority: 1,
                                                            });
                                                            match v2 {
                                                            _ => println!("break at d5"),
                                                        }
                                                    }
												}
											}
											_ => println!("break at d3"),
										}
									}
									'o' => {
    										//'grow'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										flex_grid.push(TailwindcssClass {
    										  value: v4,
    										  priority: 2,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									'a' => {
    										//'grayscale'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										effects.push(TailwindcssClass {
    										  value: v4,
    										  priority: 6,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							'a' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									'p' => { 
    										let d3 = init_itra.next().unwrap(); 
    										match d3 {
											'-' => { 
    												let d4 = init_itra.next().unwrap(); 
    												match d4 {
													'x' => {
    														//'gap-x'
    														let mut v2 = init_itra.next().unwrap();
    														let mut v3 = String::new();
    														// println!("Scale Valie{}", v2);
    														while v2 != ' ' {
    														    v3 = v3 + &String::from(v2);
    														    match init_itra.next() {
    														        Some(next_char_value) => v2 = next_char_value,
    														        None => break,
    														    }
    														}
    														let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string() + &v3; 
    														flex_grid.push(TailwindcssClass {
    														  value: v4,
    														  priority: 9,
    														});
    														match v2 {
															_ => println!("break at d5"),
														}
													}
													'y' => {
    														//'gap-y'
    														let mut v2 = init_itra.next().unwrap();
    														let mut v3 = String::new();
    														// println!("Scale Valie{}", v2);
    														while v2 != ' ' {
    														    v3 = v3 + &String::from(v2);
    														    match init_itra.next() {
    														        Some(next_char_value) => v2 = next_char_value,
    														        None => break,
    														    }
    														}
    														let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string() + &v3; 
    														flex_grid.push(TailwindcssClass {
    														  value: v4,
    														  priority: 10,
    														});
    														match v2 {
															_ => println!("break at d5"),
														}
													}
                                                    _ => {
                                                            //'gap-'
                                                            let mut v2 = init_itra.next().unwrap();
                                                            let mut v3 = String::new();
                                                            // println!("Scale Valie{}", v2);
                                                            while v2 != ' ' {
                                                                v3 = v3 + &String::from(v2);
                                                                match init_itra.next() {
                                                                    Some(next_char_value) => v2 = next_char_value,
                                                                    None => break,
                                                                }
                                                            }
                                                            let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string() + &v3; 
                                                            flex_grid.push(TailwindcssClass {
                                                              value: v4,
                                                              priority: 8,
                                                            });
                                                            match v2 {
                                                            _ => println!("break at d5"),
                                                        }
                                                    }
												}
											}
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							_ => println!("break at d1"),
						}
					}
					'h' => { 
    						let d1 = init_itra.next().unwrap(); 
    						match d1 {
							'i' => {
    								//'hidden'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								box_model.push(TailwindcssClass {
    								  value: v4,
    								  priority: 1,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							'-' => {
    								//'h-'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								box_model.push(TailwindcssClass {
    								  value: v4,
    								  priority: 7,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							'u' => {
    								//'hue'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								effects.push(TailwindcssClass {
    								  value: v4,
    								  priority: 7,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							_ => println!("break at d1"),
						}
					}
					'i' => { 
    						let d1 = init_itra.next().unwrap(); 
    						match d1 {
							'n' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									's' => {
    										//'inset'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										positioning.push(TailwindcssClass {
    										  value: v4,
    										  priority: 2,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									'l' => {
    										//'inline'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										box_model.push(TailwindcssClass {
    										  value: v4,
    										  priority: 1,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									'd' => {
    										//'indent'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										typography.push(TailwindcssClass {
    										  value: v4,
    										  priority: 13,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									'v' => {
    										//'invert'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										effects.push(TailwindcssClass {
    										  value: v4,
    										  priority: 8,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							's' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									'o' => { 
    										let d3 = init_itra.next().unwrap(); 
    										match d3 {
											'l' => { 
    												let d4 = init_itra.next().unwrap(); 
    												match d4 {
													'a' => { 
    														let d5 = init_itra.next().unwrap(); 
    														match d5 {
															't' => { 
    																let d6 = init_itra.next().unwrap(); 
    																match d6 {
																	'e' => {
    																		//'isolate'
    																		let mut v2 = init_itra.next().unwrap();
    																		let mut v3 = String::new();
    																		// println!("Scale Valie{}", v2);
    																		while v2 != ' ' {
    																		    v3 = v3 + &String::from(v2);
    																		    match init_itra.next() {
    																		        Some(next_char_value) => v2 = next_char_value,
    																		        None => break,
    																		    }
    																		}
    																		let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3; 
    																		box_model.push(TailwindcssClass {
    																		  value: v4,
    																		  priority: 0,
    																		});
    																		match v2 {
																			_ => println!("break at d7"),
																		}
																	}
																	'i' => {
    																		//'isolation-auto'
    																		let mut v2 = init_itra.next().unwrap();
    																		let mut v3 = String::new();
    																		// println!("Scale Valie{}", v2);
    																		while v2 != ' ' {
    																		    v3 = v3 + &String::from(v2);
    																		    match init_itra.next() {
    																		        Some(next_char_value) => v2 = next_char_value,
    																		        None => break,
    																		    }
    																		}
    																		let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3; 
    																		box_model.push(TailwindcssClass {
    																		  value: v4,
    																		  priority: 0,
    																		});
    																		match v2 {
																			_ => println!("break at d7"),
																		}
																	}
																	_ => println!("break at d6"),
																}
															}
															_ => println!("break at d5"),
														}
													}
													_ => println!("break at d4"),
												}
											}
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							't' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									'e' => {
    										//'items'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										alignment.push(TailwindcssClass {
    										  value: v4,
    										  priority: 1,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									'a' => {
    										//'italic'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										typography.push(TailwindcssClass {
    										  value: v4,
    										  priority: 5,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							_ => println!("break at d1"),
						}
					}
					'j' => { 
    						let d1 = init_itra.next().unwrap(); 
    						match d1 {
							_ => println!("break at d1"),
						}
					}
					'l' => { 
    						let d1 = init_itra.next().unwrap(); 
    						match d1 {
							'e' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									'f' => {
    										//'left'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										positioning.push(TailwindcssClass {
    										  value: v4,
    										  priority: 6,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									'a' => {
    										//'leading'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										typography.push(TailwindcssClass {
    										  value: v4,
    										  priority: 8,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							'i' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									's' => { 
    										let d3 = init_itra.next().unwrap(); 
    										match d3 {
											't' => { 
    												let d4 = init_itra.next().unwrap(); 
    												match d4 {
													'-' => { 
    														let d5 = init_itra.next().unwrap(); 
    														match d5 {
															'i' => { 
    																let d6 = init_itra.next().unwrap(); 
    																match d6 {
																	't' => {
    																		//'list-item'
    																		let mut v2 = init_itra.next().unwrap();
    																		let mut v3 = String::new();
    																		// println!("Scale Valie{}", v2);
    																		while v2 != ' ' {
    																		    v3 = v3 + &String::from(v2);
    																		    match init_itra.next() {
    																		        Some(next_char_value) => v2 = next_char_value,
    																		        None => break,
    																		    }
    																		}
    																		let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3; 
    																		box_model.push(TailwindcssClass {
    																		  value: v4,
    																		  priority: 1,
    																		});
    																		match v2 {
																			_ => println!("break at d7"),
																		}
																	}
																	'n' => {
    																		//'list-inside'
    																		let mut v2 = init_itra.next().unwrap();
    																		let mut v3 = String::new();
    																		// println!("Scale Valie{}", v2);
    																		while v2 != ' ' {
    																		    v3 = v3 + &String::from(v2);
    																		    match init_itra.next() {
    																		        Some(next_char_value) => v2 = next_char_value,
    																		        None => break,
    																		    }
    																		}
    																		let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3; 
    																		typography.push(TailwindcssClass {
    																		  value: v4,
    																		  priority: 3,
    																		});
    																		match v2 {
																			_ => println!("break at d7"),
																		}
																	}
																	_ => println!("break at d6"),
																}
															}
															'n' => {
    																//'list-none'
    																let mut v2 = init_itra.next().unwrap();
    																let mut v3 = String::new();
    																// println!("Scale Valie{}", v2);
    																while v2 != ' ' {
    																    v3 = v3 + &String::from(v2);
    																    match init_itra.next() {
    																        Some(next_char_value) => v2 = next_char_value,
    																        None => break,
    																    }
    																}
    																let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string() + &v3; 
    																typography.push(TailwindcssClass {
    																  value: v4,
    																  priority: 3,
    																});
    																match v2 {
																	_ => println!("break at d6"),
																}
															}
															'd' => { 
    																let d6 = init_itra.next().unwrap(); 
    																match d6 {
																	'i' => {
    																		//'list-disc'
    																		let mut v2 = init_itra.next().unwrap();
    																		let mut v3 = String::new();
    																		// println!("Scale Valie{}", v2);
    																		while v2 != ' ' {
    																		    v3 = v3 + &String::from(v2);
    																		    match init_itra.next() {
    																		        Some(next_char_value) => v2 = next_char_value,
    																		        None => break,
    																		    }
    																		}
    																		let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3; 
    																		typography.push(TailwindcssClass {
    																		  value: v4,
    																		  priority: 3,
    																		});
    																		match v2 {
																			_ => println!("break at d7"),
																		}
																	}
																	'e' => {
    																		//'list-decimal'
    																		let mut v2 = init_itra.next().unwrap();
    																		let mut v3 = String::new();
    																		// println!("Scale Valie{}", v2);
    																		while v2 != ' ' {
    																		    v3 = v3 + &String::from(v2);
    																		    match init_itra.next() {
    																		        Some(next_char_value) => v2 = next_char_value,
    																		        None => break,
    																		    }
    																		}
    																		let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3; 
    																		typography.push(TailwindcssClass {
    																		  value: v4,
    																		  priority: 3,
    																		});
    																		match v2 {
																			_ => println!("break at d7"),
																		}
																	}
																	_ => println!("break at d6"),
																}
															}
															'o' => {
    																//'list-outside'
    																let mut v2 = init_itra.next().unwrap();
    																let mut v3 = String::new();
    																// println!("Scale Valie{}", v2);
    																while v2 != ' ' {
    																    v3 = v3 + &String::from(v2);
    																    match init_itra.next() {
    																        Some(next_char_value) => v2 = next_char_value,
    																        None => break,
    																    }
    																}
    																let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string() + &v3; 
    																typography.push(TailwindcssClass {
    																  value: v4,
    																  priority: 3,
    																});
    																match v2 {
																	_ => println!("break at d6"),
																}
															}
															_ => println!("break at d5"),
														}
													}
													_ => println!("break at d4"),
												}
											}
											_ => println!("break at d3"),
										}
									}
									'n' => {
    										//'lining-nums'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										typography.push(TailwindcssClass {
    										  value: v4,
    										  priority: 9,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							'o' => {
    								//'lowercase'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								typography.push(TailwindcssClass {
    								  value: v4,
    								  priority: 11,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							_ => println!("break at d1"),
						}
					}
					'm' => { 
    						let d1 = init_itra.next().unwrap(); 
    						match d1 {
							'a' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									'x' => { 
    										let d3 = init_itra.next().unwrap(); 
    										match d3 {
											'-' => { 
    												let d4 = init_itra.next().unwrap(); 
    												match d4 {
													'w' => {
    														//'max-w'
    														let mut v2 = init_itra.next().unwrap();
    														let mut v3 = String::new();
    														// println!("Scale Valie{}", v2);
    														while v2 != ' ' {
    														    v3 = v3 + &String::from(v2);
    														    match init_itra.next() {
    														        Some(next_char_value) => v2 = next_char_value,
    														        None => break,
    														    }
    														}
    														let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string() + &v3; 
    														box_model.push(TailwindcssClass {
    														  value: v4,
    														  priority: 5,
    														});
    														match v2 {
															_ => println!("break at d5"),
														}
													}
													'h' => {
    														//'max-h'
    														let mut v2 = init_itra.next().unwrap();
    														let mut v3 = String::new();
    														// println!("Scale Valie{}", v2);
    														while v2 != ' ' {
    														    v3 = v3 + &String::from(v2);
    														    match init_itra.next() {
    														        Some(next_char_value) => v2 = next_char_value,
    														        None => break,
    														    }
    														}
    														let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string() + &v3; 
    														box_model.push(TailwindcssClass {
    														  value: v4,
    														  priority: 8,
    														});
    														match v2 {
															_ => println!("break at d5"),
														}
													}
													_ => println!("break at d4"),
												}
											}
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							'i' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									'n' => { 
    										let d3 = init_itra.next().unwrap(); 
    										match d3 {
											'-' => { 
    												let d4 = init_itra.next().unwrap(); 
    												match d4 {
													'w' => {
    														//'min-w'
    														let mut v2 = init_itra.next().unwrap();
    														let mut v3 = String::new();
    														// println!("Scale Valie{}", v2);
    														while v2 != ' ' {
    														    v3 = v3 + &String::from(v2);
    														    match init_itra.next() {
    														        Some(next_char_value) => v2 = next_char_value,
    														        None => break,
    														    }
    														}
    														let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string() + &v3; 
    														box_model.push(TailwindcssClass {
    														  value: v4,
    														  priority: 6,
    														});
    														match v2 {
															_ => println!("break at d5"),
														}
													}
													'h' => {
    														//'min-h'
    														let mut v2 = init_itra.next().unwrap();
    														let mut v3 = String::new();
    														// println!("Scale Valie{}", v2);
    														while v2 != ' ' {
    														    v3 = v3 + &String::from(v2);
    														    match init_itra.next() {
    														        Some(next_char_value) => v2 = next_char_value,
    														        None => break,
    														    }
    														}
    														let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string() + &v3; 
    														box_model.push(TailwindcssClass {
    														  value: v4,
    														  priority: 9,
    														});
    														match v2 {
															_ => println!("break at d5"),
														}
													}
													_ => println!("break at d4"),
												}
											}
											_ => println!("break at d3"),
										}
									}
									'x' => {
    										//'mix-blend'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										effects.push(TailwindcssClass {
    										  value: v4,
    										  priority: 2,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							'-' => {
    								//'m-'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								box_model.push(TailwindcssClass {
    								  value: v4,
    								  priority: 41,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							'x' => {
    								//'mx'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								box_model.push(TailwindcssClass {
    								  value: v4,
    								  priority: 42,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							'y' => {
    								//'my'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								box_model.push(TailwindcssClass {
    								  value: v4,
    								  priority: 43,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							't' => {
    								//'mt'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								box_model.push(TailwindcssClass {
    								  value: v4,
    								  priority: 44,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							'l' => {
    								//'ml'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								box_model.push(TailwindcssClass {
    								  value: v4,
    								  priority: 45,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							'r' => {
    								//'mr'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								box_model.push(TailwindcssClass {
    								  value: v4,
    								  priority: 46,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							'b' => {
    								//'mb'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								box_model.push(TailwindcssClass {
    								  value: v4,
    								  priority: 47,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							_ => println!("break at d1"),
						}
					}
					'n' => { 
    						let d1 = init_itra.next().unwrap(); 
    						match d1 {
							'o' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									't' => { 
    										let d3 = init_itra.next().unwrap(); 
    										match d3 {
											'-' => { 
    												let d4 = init_itra.next().unwrap(); 
    												match d4 {
													'i' => {
    														//'not-italic'
    														let mut v2 = init_itra.next().unwrap();
    														let mut v3 = String::new();
    														// println!("Scale Valie{}", v2);
    														while v2 != ' ' {
    														    v3 = v3 + &String::from(v2);
    														    match init_itra.next() {
    														        Some(next_char_value) => v2 = next_char_value,
    														        None => break,
    														    }
    														}
    														let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string() + &v3; 
    														typography.push(TailwindcssClass {
    														  value: v4,
    														  priority: 6,
    														});
    														match v2 {
															_ => println!("break at d5"),
														}
													}
													's' => {
    														//'not-sr-only'
    														let mut v2 = init_itra.next().unwrap();
    														let mut v3 = String::new();
    														// println!("Scale Valie{}", v2);
    														while v2 != ' ' {
    														    v3 = v3 + &String::from(v2);
    														    match init_itra.next() {
    														        Some(next_char_value) => v2 = next_char_value,
    														        None => break,
    														    }
    														}
    														let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string() + &v3; 
    														misc.push(TailwindcssClass {
    														  value: v4,
    														  priority: 1,
    														});
    														match v2 {
															_ => println!("break at d5"),
														}
													}
													_ => println!("break at d4"),
												}
											}
											_ => println!("break at d3"),
										}
									}
									'r' => { 
    										let d3 = init_itra.next().unwrap(); 
    										match d3 {
											'm' => { 
    												let d4 = init_itra.next().unwrap(); 
    												match d4 {
													'a' => { 
    														let d5 = init_itra.next().unwrap(); 
    														match d5 {
															'l' => { 
    																let d6 = init_itra.next().unwrap(); 
    																match d6 {
																	'-' => { 
    																		let d7 = init_itra.next().unwrap(); 
    																		match d7 {
																			'n' => {
    																				//'normal-nums'
    																				let mut v2 = init_itra.next().unwrap();
    																				let mut v3 = String::new();
    																				// println!("Scale Valie{}", v2);
    																				while v2 != ' ' {
    																				    v3 = v3 + &String::from(v2);
    																				    match init_itra.next() {
    																				        Some(next_char_value) => v2 = next_char_value,
    																				        None => break,
    																				    }
    																				}
    																				let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3; 
    																				typography.push(TailwindcssClass {
    																				  value: v4,
    																				  priority: 9,
    																				});
    																				match v2 {
																					_ => println!("break at d8"),
																				}
																			}
																			'c' => {
    																				//'normal-case'
    																				let mut v2 = init_itra.next().unwrap();
    																				let mut v3 = String::new();
    																				// println!("Scale Valie{}", v2);
    																				while v2 != ' ' {
    																				    v3 = v3 + &String::from(v2);
    																				    match init_itra.next() {
    																				        Some(next_char_value) => v2 = next_char_value,
    																				        None => break,
    																				    }
    																				}
    																				let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string()+ &d7.to_string() + &v3; 
    																				typography.push(TailwindcssClass {
    																				  value: v4,
    																				  priority: 11,
    																				});
    																				match v2 {
																					_ => println!("break at d8"),
																				}
																			}
																			_ => println!("break at d7"),
																		}
																	}
																	_ => println!("break at d6"),
																}
															}
															_ => println!("break at d5"),
														}
													}
													_ => println!("break at d4"),
												}
											}
											_ => println!("break at d3"),
										}
									}
									'-' => {
    										//'no-underline'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										typography.push(TailwindcssClass {
    										  value: v4,
    										  priority: 10,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							_ => println!("break at d1"),
						}
					}
					'o' => { 
    						let d1 = init_itra.next().unwrap(); 
    						match d1 {
							'b' => {
    								//'object'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								positioning.push(TailwindcssClass {
    								  value: v4,
    								  priority: 7,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							'v' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									'e' => { 
    										let d3 = init_itra.next().unwrap(); 
    										match d3 {
											'r' => { 
    												let d4 = init_itra.next().unwrap(); 
    												match d4 {
													'f' => {
    														//'overflow'
    														let mut v2 = init_itra.next().unwrap();
    														let mut v3 = String::new();
    														// println!("Scale Valie{}", v2);
    														while v2 != ' ' {
    														    v3 = v3 + &String::from(v2);
    														    match init_itra.next() {
    														        Some(next_char_value) => v2 = next_char_value,
    														        None => break,
    														    }
    														}
    														let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string() + &v3; 
    														box_model.push(TailwindcssClass {
    														  value: v4,
    														  priority: 99,
    														});
    														match v2 {
															_ => println!("break at d5"),
														}
													}
													's' => {
    														//'overscroll'
    														let mut v2 = init_itra.next().unwrap();
    														let mut v3 = String::new();
    														// println!("Scale Valie{}", v2);
    														while v2 != ' ' {
    														    v3 = v3 + &String::from(v2);
    														    match init_itra.next() {
    														        Some(next_char_value) => v2 = next_char_value,
    														        None => break,
    														    }
    														}
    														let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string() + &v3; 
    														box_model.push(TailwindcssClass {
    														  value: v4,
    														  priority: 99,
    														});
    														match v2 {
															_ => println!("break at d5"),
														}
													}
													'l' => {
    														//'overline'
    														let mut v2 = init_itra.next().unwrap();
    														let mut v3 = String::new();
    														// println!("Scale Valie{}", v2);
    														while v2 != ' ' {
    														    v3 = v3 + &String::from(v2);
    														    match init_itra.next() {
    														        Some(next_char_value) => v2 = next_char_value,
    														        None => break,
    														    }
    														}
    														let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string() + &v3; 
    														typography.push(TailwindcssClass {
    														  value: v4,
    														  priority: 10,
    														});
    														match v2 {
															_ => println!("break at d5"),
														}
													}
													_ => println!("break at d4"),
												}
											}
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							'u' => {
    								//'outline'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								box_model.push(TailwindcssClass {
    								  value: v4,
    								  priority: 22,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							'r' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									'd' => { 
    										let d3 = init_itra.next().unwrap(); 
    										match d3 {
											'e' => {
    												//'order'
    												let mut v2 = init_itra.next().unwrap();
    												let mut v3 = String::new();
    												// println!("Scale Valie{}", v2);
    												while v2 != ' ' {
    												    v3 = v3 + &String::from(v2);
    												    match init_itra.next() {
    												        Some(next_char_value) => v2 = next_char_value,
    												        None => break,
    												    }
    												}
    												let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string() + &v3; 
    												flex_grid.push(TailwindcssClass {
    												  value: v4,
    												  priority: 4,
    												});
    												match v2 {
													_ => println!("break at d4"),
												}
											}
											'i' => {
    												//'ordinal'
    												let mut v2 = init_itra.next().unwrap();
    												let mut v3 = String::new();
    												// println!("Scale Valie{}", v2);
    												while v2 != ' ' {
    												    v3 = v3 + &String::from(v2);
    												    match init_itra.next() {
    												        Some(next_char_value) => v2 = next_char_value,
    												        None => break,
    												    }
    												}
    												let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string() + &v3; 
    												typography.push(TailwindcssClass {
    												  value: v4,
    												  priority: 9,
    												});
    												match v2 {
													_ => println!("break at d4"),
												}
											}
											_ => println!("break at d3"),
										}
									}
									'i' => {
    										//'origin'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										transforms.push(TailwindcssClass {
    										  value: v4,
    										  priority: 1,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							'l' => {
    								//'oldstyle-nums'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								typography.push(TailwindcssClass {
    								  value: v4,
    								  priority: 9,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							'p' => {
    								//'opacity'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								effects.push(TailwindcssClass {
    								  value: v4,
    								  priority: 1,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							_ => println!("break at d1"),
						}
					}
					'p' => { 
    						let d1 = init_itra.next().unwrap(); 
    						match d1 {
							'-' => {
    								//'p-'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								box_model.push(TailwindcssClass {
    								  value: v4,
    								  priority: 11,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							'x' => {
    								//'px'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								box_model.push(TailwindcssClass {
    								  value: v4,
    								  priority: 12,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							'y' => {
    								//'py'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								box_model.push(TailwindcssClass {
    								  value: v4,
    								  priority: 13,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							't' => {
    								//'pt'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								box_model.push(TailwindcssClass {
    								  value: v4,
    								  priority: 14,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							'l' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									'-' => {
    										//'pl'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										box_model.push(TailwindcssClass {
    										  value: v4,
    										  priority: 15,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									'a' => { 
    										let d3 = init_itra.next().unwrap(); 
    										match d3 {
											'c' => { 
    												let d4 = init_itra.next().unwrap(); 
    												match d4 {
													'e' => { 
    														let d5 = init_itra.next().unwrap(); 
    														match d5 {
															'-' => { 
    																let d6 = init_itra.next().unwrap(); 
    																match d6 {
																	'c' => {
    																		//'place-content'
    																		let mut v2 = init_itra.next().unwrap();
    																		let mut v3 = String::new();
    																		// println!("Scale Valie{}", v2);
    																		while v2 != ' ' {
    																		    v3 = v3 + &String::from(v2);
    																		    match init_itra.next() {
    																		        Some(next_char_value) => v2 = next_char_value,
    																		        None => break,
    																		    }
    																		}
    																		let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3; 
    																		alignment.push(TailwindcssClass {
    																		  value: v4,
    																		  priority: 3,
    																		});
    																		match v2 {
																			_ => println!("break at d7"),
																		}
																	}
																	'i' => {
    																		//'place-items'
    																		let mut v2 = init_itra.next().unwrap();
    																		let mut v3 = String::new();
    																		// println!("Scale Valie{}", v2);
    																		while v2 != ' ' {
    																		    v3 = v3 + &String::from(v2);
    																		    match init_itra.next() {
    																		        Some(next_char_value) => v2 = next_char_value,
    																		        None => break,
    																		    }
    																		}
    																		let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3; 
    																		alignment.push(TailwindcssClass {
    																		  value: v4,
    																		  priority: 5,
    																		});
    																		match v2 {
																			_ => println!("break at d7"),
																		}
																	}
																	's' => {
    																		//'place-self'
    																		let mut v2 = init_itra.next().unwrap();
    																		let mut v3 = String::new();
    																		// println!("Scale Valie{}", v2);
    																		while v2 != ' ' {
    																		    v3 = v3 + &String::from(v2);
    																		    match init_itra.next() {
    																		        Some(next_char_value) => v2 = next_char_value,
    																		        None => break,
    																		    }
    																		}
    																		let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3; 
    																		alignment.push(TailwindcssClass {
    																		  value: v4,
    																		  priority: 6,
    																		});
    																		match v2 {
																			_ => println!("break at d7"),
																		}
																	}
																	_ => println!("break at d6"),
																}
															}
															_ => println!("break at d5"),
														}
													}
													_ => println!("break at d4"),
												}
											}
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							'r' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									'-' => {
    										//'pr'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										box_model.push(TailwindcssClass {
    										  value: v4,
    										  priority: 16,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									'o' => {
    										//'proportional-nums'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										typography.push(TailwindcssClass {
    										  value: v4,
    										  priority: 9,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							'b' => {
    								//'pb'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								box_model.push(TailwindcssClass {
    								  value: v4,
    								  priority: 17,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							'o' => {
    								//'pointer'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								misc.push(TailwindcssClass {
    								  value: v4,
    								  priority: 1,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							_ => println!("break at d1"),
						}
					}
					'r' => { 
    						let d1 = init_itra.next().unwrap(); 
    						match d1 {
							'e' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									'l' => {
    										//'relative'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										positioning.push(TailwindcssClass {
    										  value: v4,
    										  priority: 1,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									's' => {
    										//'resize'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										misc.push(TailwindcssClass {
    										  value: v4,
    										  priority: 1,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							'i' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									'g' => {
    										//'right'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										positioning.push(TailwindcssClass {
    										  value: v4,
    										  priority: 4,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									'n' => {
    										//'ring'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										box_model.push(TailwindcssClass {
    										  value: v4,
    										  priority: 23,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							'o' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									'u' => {
    										//'rounded'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										box_model.push(TailwindcssClass {
    										  value: v4,
    										  priority: 32,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									'w' => {
    										//'row'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										flex_grid.push(TailwindcssClass {
    										  value: v4,
    										  priority: 2,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									't' => {
    										//'rotate'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										transforms.push(TailwindcssClass {
    										  value: v4,
    										  priority: 5,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							_ => println!("break at d1"),
						}
					}
					's' => { 
    						let d1 = init_itra.next().unwrap(); 
    						match d1 {
							't' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									'a' => { 
    										let d3 = init_itra.next().unwrap(); 
    										match d3 {
											't' => {
    												//'static'
    												let mut v2 = init_itra.next().unwrap();
    												let mut v3 = String::new();
    												// println!("Scale Valie{}", v2);
    												while v2 != ' ' {
    												    v3 = v3 + &String::from(v2);
    												    match init_itra.next() {
    												        Some(next_char_value) => v2 = next_char_value,
    												        None => break,
    												    }
    												}
    												let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string() + &v3; 
    												positioning.push(TailwindcssClass {
    												  value: v4,
    												  priority: 1,
    												});
    												match v2 {
													_ => println!("break at d4"),
												}
											}
											'c' => {
    												//'stacked-fractions'
    												let mut v2 = init_itra.next().unwrap();
    												let mut v3 = String::new();
    												// println!("Scale Valie{}", v2);
    												while v2 != ' ' {
    												    v3 = v3 + &String::from(v2);
    												    match init_itra.next() {
    												        Some(next_char_value) => v2 = next_char_value,
    												        None => break,
    												    }
    												}
    												let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string() + &v3; 
    												typography.push(TailwindcssClass {
    												  value: v4,
    												  priority: 9,
    												});
    												match v2 {
													_ => println!("break at d4"),
												}
											}
											_ => println!("break at d3"),
										}
									}
									'i' => {
    										//'sticky'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										positioning.push(TailwindcssClass {
    										  value: v4,
    										  priority: 1,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									'r' => { 
    										let d3 = init_itra.next().unwrap(); 
    										match d3 {
											'i' => {
    												//'strike-through'
    												let mut v2 = init_itra.next().unwrap();
    												let mut v3 = String::new();
    												// println!("Scale Valie{}", v2);
    												while v2 != ' ' {
    												    v3 = v3 + &String::from(v2);
    												    match init_itra.next() {
    												        Some(next_char_value) => v2 = next_char_value,
    												        None => break,
    												    }
    												}
    												let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string() + &v3; 
    												typography.push(TailwindcssClass {
    												  value: v4,
    												  priority: 10,
    												});
    												match v2 {
													_ => println!("break at d4"),
												}
											}
											'o' => {
    												//'stroke'
    												let mut v2 = init_itra.next().unwrap();
    												let mut v3 = String::new();
    												// println!("Scale Valie{}", v2);
    												while v2 != ' ' {
    												    v3 = v3 + &String::from(v2);
    												    match init_itra.next() {
    												        Some(next_char_value) => v2 = next_char_value,
    												        None => break,
    												    }
    												}
    												let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string() + &v3; 
    												visuals.push(TailwindcssClass {
    												  value: v4,
    												  priority: 2,
    												});
    												match v2 {
													_ => println!("break at d4"),
												}
											}
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							'p' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									'a' => { 
    										let d3 = init_itra.next().unwrap(); 
    										match d3 {
											'c' => { 
    												let d4 = init_itra.next().unwrap(); 
    												match d4 {
													'e' => { 
    														let d5 = init_itra.next().unwrap(); 
    														match d5 {
															'-' => { 
    																let d6 = init_itra.next().unwrap(); 
    																match d6 {
																	'x' => {
    																		//'space-x'
    																		let mut v2 = init_itra.next().unwrap();
    																		let mut v3 = String::new();
    																		// println!("Scale Valie{}", v2);
    																		while v2 != ' ' {
    																		    v3 = v3 + &String::from(v2);
    																		    match init_itra.next() {
    																		        Some(next_char_value) => v2 = next_char_value,
    																		        None => break,
    																		    }
    																		}
    																		let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3; 
    																		box_model.push(TailwindcssClass {
    																		  value: v4,
    																		  priority: 48,
    																		});
    																		match v2 {
																			_ => println!("break at d7"),
																		}
																	}
																	'y' => {
    																		//'space-y'
    																		let mut v2 = init_itra.next().unwrap();
    																		let mut v3 = String::new();
    																		// println!("Scale Valie{}", v2);
    																		while v2 != ' ' {
    																		    v3 = v3 + &String::from(v2);
    																		    match init_itra.next() {
    																		        Some(next_char_value) => v2 = next_char_value,
    																		        None => break,
    																		    }
    																		}
    																		let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string()+ &d6.to_string() + &v3; 
    																		box_model.push(TailwindcssClass {
    																		  value: v4,
    																		  priority: 49,
    																		});
    																		match v2 {
																			_ => println!("break at d7"),
																		}
																	}
																	_ => println!("break at d6"),
																}
															}
															_ => println!("break at d5"),
														}
													}
													_ => println!("break at d4"),
												}
											}
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							'e' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									'l' => { 
    										let d3 = init_itra.next().unwrap(); 
    										match d3 {
											'f' => {
    												//'self'
    												let mut v2 = init_itra.next().unwrap();
    												let mut v3 = String::new();
    												// println!("Scale Valie{}", v2);
    												while v2 != ' ' {
    												    v3 = v3 + &String::from(v2);
    												    match init_itra.next() {
    												        Some(next_char_value) => v2 = next_char_value,
    												        None => break,
    												    }
    												}
    												let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string() + &v3; 
    												alignment.push(TailwindcssClass {
    												  value: v4,
    												  priority: 4,
    												});
    												match v2 {
													_ => println!("break at d4"),
												}
											}
											'e' => {
    												//'select'
    												let mut v2 = init_itra.next().unwrap();
    												let mut v3 = String::new();
    												// println!("Scale Valie{}", v2);
    												while v2 != ' ' {
    												    v3 = v3 + &String::from(v2);
    												    match init_itra.next() {
    												        Some(next_char_value) => v2 = next_char_value,
    												        None => break,
    												    }
    												}
    												let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string() + &v3; 
    												misc.push(TailwindcssClass {
    												  value: v4,
    												  priority: 1,
    												});
    												match v2 {
													_ => println!("break at d4"),
												}
											}
											_ => println!("break at d3"),
										}
									}
									'p' => {
    										//'sepia'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										effects.push(TailwindcssClass {
    										  value: v4,
    										  priority: 10,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							'h' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									'r' => {
    										//'shrink'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										flex_grid.push(TailwindcssClass {
    										  value: v4,
    										  priority: 3,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									'a' => {
    										//'shadow'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										effects.push(TailwindcssClass {
    										  value: v4,
    										  priority: 1,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							'u' => {
    								//'subpixel-antialiased'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								typography.push(TailwindcssClass {
    								  value: v4,
    								  priority: 4,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							'l' => {
    								//'slashed-zero'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								typography.push(TailwindcssClass {
    								  value: v4,
    								  priority: 9,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							'a' => {
    								//'saturate'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								effects.push(TailwindcssClass {
    								  value: v4,
    								  priority: 9,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							'c' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									'a' => {
    										//'scale'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										transforms.push(TailwindcssClass {
    										  value: v4,
    										  priority: 3,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									'o' => {
    										//'scoll'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										misc.push(TailwindcssClass {
    										  value: v4,
    										  priority: 1,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							'k' => {
    								//'skew'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								transforms.push(TailwindcssClass {
    								  value: v4,
    								  priority: 4,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							'n' => {
    								//'snap'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								misc.push(TailwindcssClass {
    								  value: v4,
    								  priority: 1,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							'r' => {
    								//'sr-only'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								misc.push(TailwindcssClass {
    								  value: v4,
    								  priority: 1,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							_ => println!("break at d1"),
						}
					}
					't' => { 
    						let d1 = init_itra.next().unwrap(); 
    						match d1 {
							'o' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									'p' => {
    										//'top'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										positioning.push(TailwindcssClass {
    										  value: v4,
    										  priority: 3,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									'-' => {
    										//'to-'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										visuals.push(TailwindcssClass {
    										  value: v4,
    										  priority: 14,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									'u' => {
    										//'touch'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										misc.push(TailwindcssClass {
    										  value: v4,
    										  priority: 1,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							'a' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									'b' => { 
    										let d3 = init_itra.next().unwrap(); 
    										match d3 {
											'l' => {
    												//'table'
    												let mut v2 = init_itra.next().unwrap();
    												let mut v3 = String::new();
    												// println!("Scale Valie{}", v2);
    												while v2 != ' ' {
    												    v3 = v3 + &String::from(v2);
    												    match init_itra.next() {
    												        Some(next_char_value) => v2 = next_char_value,
    												        None => break,
    												    }
    												}
    												let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string() + &v3; 
    												box_model.push(TailwindcssClass {
    												  value: v4,
    												  priority: 1,
    												});
    												match v2 {
													_ => println!("break at d4"),
												}
											}
											'u' => {
    												//'tabular-nums'
    												let mut v2 = init_itra.next().unwrap();
    												let mut v3 = String::new();
    												// println!("Scale Valie{}", v2);
    												while v2 != ' ' {
    												    v3 = v3 + &String::from(v2);
    												    match init_itra.next() {
    												        Some(next_char_value) => v2 = next_char_value,
    												        None => break,
    												    }
    												}
    												let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string() + &v3; 
    												typography.push(TailwindcssClass {
    												  value: v4,
    												  priority: 9,
    												});
    												match v2 {
													_ => println!("break at d4"),
												}
											}
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							'e' => {
    								//'text-'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								typography.push(TailwindcssClass {
    								  value: v4,
    								  priority: 2,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							'r' => { 
    								let d2 = init_itra.next().unwrap(); 
    								match d2 {
									'a' => { 
    										let d3 = init_itra.next().unwrap(); 
    										match d3 {
											'c' => {
    												//'tracking'
    												let mut v2 = init_itra.next().unwrap();
    												let mut v3 = String::new();
    												// println!("Scale Valie{}", v2);
    												while v2 != ' ' {
    												    v3 = v3 + &String::from(v2);
    												    match init_itra.next() {
    												        Some(next_char_value) => v2 = next_char_value,
    												        None => break,
    												    }
    												}
    												let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string() + &v3; 
    												typography.push(TailwindcssClass {
    												  value: v4,
    												  priority: 7,
    												});
    												match v2 {
													_ => println!("break at d4"),
												}
											}
											'n' => { 
    												let d4 = init_itra.next().unwrap(); 
    												match d4 {
													's' => { 
    														let d5 = init_itra.next().unwrap(); 
    														match d5 {
															'l' => {
    																//'translate'
    																let mut v2 = init_itra.next().unwrap();
    																let mut v3 = String::new();
    																// println!("Scale Valie{}", v2);
    																while v2 != ' ' {
    																    v3 = v3 + &String::from(v2);
    																    match init_itra.next() {
    																        Some(next_char_value) => v2 = next_char_value,
    																        None => break,
    																    }
    																}
    																let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string() + &v3; 
    																transforms.push(TailwindcssClass {
    																  value: v4,
    																  priority: 2,
    																});
    																match v2 {
																	_ => println!("break at d6"),
																}
															}
															'i' => {
    																//'transition'
    																let mut v2 = init_itra.next().unwrap();
    																let mut v3 = String::new();
    																// println!("Scale Valie{}", v2);
    																while v2 != ' ' {
    																    v3 = v3 + &String::from(v2);
    																    match init_itra.next() {
    																        Some(next_char_value) => v2 = next_char_value,
    																        None => break,
    																    }
    																}
    																let v4 = v.to_string() + &d1.to_string()+ &d2.to_string()+ &d3.to_string()+ &d4.to_string()+ &d5.to_string() + &v3; 
    																motion.push(TailwindcssClass {
    																  value: v4,
    																  priority: 2,
    																});
    																match v2 {
																	_ => println!("break at d6"),
																}
															}
															_ => println!("break at d5"),
														}
													}
													_ => println!("break at d4"),
												}
											}
											_ => println!("break at d3"),
										}
									}
									'u' => {
    										//'truncate'
    										let mut v2 = init_itra.next().unwrap();
    										let mut v3 = String::new();
    										// println!("Scale Valie{}", v2);
    										while v2 != ' ' {
    										    v3 = v3 + &String::from(v2);
    										    match init_itra.next() {
    										        Some(next_char_value) => v2 = next_char_value,
    										        None => break,
    										    }
    										}
    										let v4 = v.to_string() + &d1.to_string()+ &d2.to_string() + &v3; 
    										typography.push(TailwindcssClass {
    										  value: v4,
    										  priority: 12,
    										});
    										match v2 {
											_ => println!("break at d3"),
										}
									}
									_ => println!("break at d2"),
								}
							}
							_ => println!("break at d1"),
						}
					}
					'u' => { 
    						let d1 = init_itra.next().unwrap(); 
    						match d1 {
							'n' => {
    								//'underline'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								typography.push(TailwindcssClass {
    								  value: v4,
    								  priority: 10,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							'p' => {
    								//'uppercase'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								typography.push(TailwindcssClass {
    								  value: v4,
    								  priority: 11,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							_ => println!("break at d1"),
						}
					}
					'v' => { 
    						let d1 = init_itra.next().unwrap(); 
    						match d1 {
							_ => println!("break at d1"),
						}
					}
					'w' => { 
    						let d1 = init_itra.next().unwrap(); 
    						match d1 {
							'-' => {
    								//'w-'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								box_model.push(TailwindcssClass {
    								  value: v4,
    								  priority: 4,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							'h' => {
    								//'whitespace'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								typography.push(TailwindcssClass {
    								  value: v4,
    								  priority: 15,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							'i' => {
    								//'will-change'
    								let mut v2 = init_itra.next().unwrap();
    								let mut v3 = String::new();
    								// println!("Scale Valie{}", v2);
    								while v2 != ' ' {
    								    v3 = v3 + &String::from(v2);
    								    match init_itra.next() {
    								        Some(next_char_value) => v2 = next_char_value,
    								        None => break,
    								    }
    								}
    								let v4 = v.to_string() + &d1.to_string() + &v3; 
    								misc.push(TailwindcssClass {
    								  value: v4,
    								  priority: 1,
    								});
    								match v2 {
									_ => println!("break at d2"),
								}
							}
							_ => println!("break at d1"),
						}
					}
					'z' => { 
    						let d1 = init_itra.next().unwrap(); 
    						match d1 {
							_ => println!("break at d1"),
						}
					}
                    _ => println!("_{}_", v),
                }

                print!("{}", v);
            }
            None => {
                println!("Loop breaked");
                break;
            }
        }
    }
	
	let duration = start.elapsed();

    println!("box_model {:#?}", box_model);

    println!("positioning {:#?}", positioning);

    println!("alignment {:#?}", alignment);

    println!("flex_grid {:#?}", flex_grid);

    println!("typography {:#?}", typography);
    // Visuals :- Classes like bg, shadows, backdrop-shadows
    println!("visuals {:#?}", visuals);
    // Effects brightness blur filters as well mix
    println!("effects {:#?}", effects);
    // Transform
    println!("transforms {:#?}", transforms);
    // ANimation & Transition
    println!("motion {:#?}", motion);
    // Misc will contain classes like caret color or scroll snap classes;
    println!("misc {:#?}", misc);

	println!("Total Loop Duration {:#?}", duration );

    // println!("Layout :- {:#?}", layout);
    // println!("Misc :- {:#?}", misc);
    // println!("Look :- {:#?}", look);
    // println!("Any :- {:#?}", any);

    // let mut final_class = String::new();
    // /* Sizing */
    // let sizing = ["w", "h", "max", "min"];
    // /* Box Model */
    // let box_model = ["m", "p", "rounded", "border", "divide", "outline", "ring"];
    // /* Flexbox-Grid */
    // let flexbox = ["basis", "flex", "grow", "shrink"];
    // let grid = ["grid", "col", "row", "auto-cols"];
    // let alignment = ["place", "self", "items", "content", "justify"];

    // /* Typography */
    // let fonts = [
    //     "font",
    //     "italic",
    //     "not-italic",
    //     "antialiased",
    //     "subpixel-antialiased",
    // ];

    // let text = [
    //     "text",
    //     "underline",
    //     "overline",
    //     "line-through",
    //     "no-underline",
    //     "decoration",
    //     "uppercase",
    //     "lowercase",
    //     "capitalize",
    //     "normal-case",
    //     "truncate",
    //     "indent",
    //     "align",
    //     "whitespace",
    //     "break-normal",
    //     "break-words",
    //     "break-all",
    //     "tracking",
    //     "leading",
    // ];

    // let lists = ["list"];
    // let background = ["bg", "from", "via", "to"];

    // /* Visuals */
    // let shadows = ["shadow"];

    // let mut intermediate_class = String::new();
    // let mut inter_mediate = String::new();

    // /* For Font */
    // for class_value in INITIAL_CLASS.split_ascii_whitespace() {
    //     let (first, _) = class_value.split_once("-").unwrap();
    //     // println!("Boolean {} {}", sizing.contains(&first), _);
    //     if fonts.contains(&first) {
    //         final_class.insert_str(0, class_value);
    //         final_class.insert_str(0, " ");
    //     } else {
    //         intermediate_class.push_str(class_value);
    //         intermediate_class.push_str(" ");
    //     }
    // }
    // final_class.insert_str(0, "\n");
    // println!(
    //     "Intermediate Class {}, \n Final Class {} \n\n",
    //     intermediate_class, final_class
    // );
    // /* For Text */
    // for class_value in intermediate_class.split_ascii_whitespace() {
    //     let (first, _) = class_value.split_once("-").unwrap();
    //     // println!("Boolean {} {}", sizing.contains(&first), _);
    //     if text.contains(&first) {
    //         final_class.insert_str(0, class_value);
    //         final_class.insert_str(0, " ");
    //     } else {
    //         inter_mediate.push_str(class_value);
    //         inter_mediate.push_str(" ");
    //     }
    // }
    // final_class.insert_str(0, "\n");
    // intermediate_class = inter_mediate;
    // inter_mediate = String::new();
    // println!(
    //     "Intermediate Class {}, \n Final Class {} \n\n",
    //     intermediate_class, final_class
    // );
    // /* Box Model */
    // for class_value in intermediate_class.split_ascii_whitespace() {
    //     let (first, _) = class_value.split_once("-").unwrap();
    //     // println!("Boolean {} {}", sizing.contains(&first), _);
    //     if box_model.contains(&first) {
    //         final_class.insert_str(0, class_value);
    //         final_class.insert_str(0, " ");
    //     } else {
    //         inter_mediate.push_str(class_value);
    //         inter_mediate.push_str(" ");
    //     }
    // }
    // final_class.insert_str(0, "\n");
    // intermediate_class = inter_mediate;
    // inter_mediate = String::new();
    // println!(
    //     "Intermediate Class {}, \n Final Class {} \n\n",
    //     intermediate_class, final_class
    // );
    // /* For Sizing */
    // for class_value in intermediate_class.split_ascii_whitespace() {
    //     let (first, _) = class_value.split_once("-").unwrap();
    //     // println!("Boolean {} {}", sizing.contains(&first), _);
    //     if sizing.contains(&first) {
    //         final_class.insert_str(0, class_value);
    //         final_class.insert_str(0, " ");
    //     } else {
    //         inter_mediate.push_str(class_value);
    //         inter_mediate.push_str(" ");
    //     }
    // }
    // final_class.insert_str(0, "\n");

    // println!("{} ", final_class)
}
