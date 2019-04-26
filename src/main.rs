extern crate rand;
extern crate promptly;

use std::env::args;
use std::process::exit;

use rand::random;
use promptly::prompt;
use rand::thread_rng;
use rand::seq::SliceRandom;


#[derive(Clone, Debug, Eq, PartialEq, Copy)]
enum Op {
	Plus,
	Minus,
	Krat,
	Deleno
}

impl Op {
	fn string(&self) -> &'static str {
		use Op::*;
		match *self {
			Plus => "+",
			Minus => "-",
			Krat => "*",
			Deleno => "/",
		}
	}

	fn apply(&self, a: i32, b: i32) -> i32 {
		use Op::*;
		match *self {
			Plus => a + b,
			Minus => a - b,
			Krat => a * b,
			Deleno => a / b,
		}
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Priklad(i32, i32, Op);

// funkce pro správné zabití když se něco nepovede
fn error(msg: &str) -> ! {
	eprintln!("{}", msg);
	exit(-1)
}

// jako operace se berou argumenty
fn main() {
	let mut operace = args().skip(1).map(|x| match x.as_ref() {
		"+" => Op::Plus,
		"-" => Op::Minus,
		"/" => Op::Deleno,
		"*" => Op::Krat,
		x => error(&format!("neplatná operace: {}", x))
	}).collect::<Vec<Op>>();
	operace.dedup();

	let pocet: i32 = prompt("zadejte počet příkladů");

	let mut spravne = 0;
	let mut chybne = 0;
	let mut chybne_p: Vec<(Priklad, i32)> = vec![];

	// lol
	let mut priklady = vec![match operace.get(0) {
		Some(Op::Plus) => Priklad(90, 1, Op::Plus),
		Some(Op::Minus) => Priklad(94, 2, Op::Minus),
		Some(Op::Krat) => Priklad(9, 11, Op::Krat),
		Some(Op::Deleno) => Priklad(93, 1, Op::Deleno),
		None => error("je zapotřebí mít alespoň jednu početní operaci"),
	}];

	for _ in 1..pocet {
		match random::<u8>() % 4 {
			0 if operace.contains(&Op::Plus) => {
				let mut a = (random::<u16>() % 198) as i32 - 99;
				let mut b = (random::<u16>() % 198) as i32 - 99;
				while (a + b) > 99 || (a + b) < -99 {
					a = (random::<u16>() % 198) as i32 - 99;
					b = (random::<u16>() % 198) as i32 - 99;
				}
				priklady.push(Priklad(a, b, Op::Plus))
			},
			1 if operace.contains(&Op::Minus) => {
				let mut a = (random::<u16>() % 198) as i32 - 99;
				let mut b = (random::<u16>() % 198) as i32 - 99;
				while (a - b) > 99 || (a - b) < -99 {
					a = (random::<u16>() % 198) as i32 - 99;
					b = (random::<u16>() % 198) as i32 - 99;
				}
				priklady.push(Priklad(a, b, Op::Minus))
			},
			2 if operace.contains(&Op::Deleno) => {
				let a = (random::<u16>() % 198) as i32 - 99;
				let mut d = (random::<u16>() % 98) as i32 - 49;
				if d == 0 {
					d += match random::<u8>() % 2 {
						0 => -1,
						1 => 1,
						_ => unreachable!(),
					};
				} // <- abychom nedělili nulou
				priklady.push(Priklad(a - (a % d), d, Op::Deleno))
			},
			3 if operace.contains(&Op::Krat) => {
				let mut a = (random::<u16>() % 198) as i32 - 99;
				let mut m = (random::<u16>() % 98) as i32 - 49;
				while (a * m) > 99 || (a * m) < -99 {
					a = (random::<u16>() % 198) as i32 - 99;
					m = (random::<u16>() % 98) as i32 - 49;
				}
				priklady.push(Priklad(a, m, Op::Krat))
			},
			_ => unreachable!()
		};
	}

	priklady.shuffle(&mut thread_rng());

	priklady.iter()
		.for_each(|x| {
			let v: i32 = prompt(&format!("{} {} {} =", x.0, x.2.string(), x.1));
			let vysledek = x.2.apply(x.0, x.1);

			if v == vysledek {
				spravne += 1
			} else {
				chybne += 1;
				chybne_p.push(((*x).clone(), vysledek));
			}
		});

	println!("Celkem: {}", pocet);
	println!("Správně: {}", spravne);
	println!("Chybně: {}", chybne);
	println!("Úspěšnost: {}%", (spravne as f32 / pocet as f32) * 100.);

	println!("Chybné příklady:");
	chybne_p.iter()
		.for_each(|(x, v)| {
			println!("{} {} {} = {} správně {}", x.0, x.2.string(), x.1, v, x.2.apply(x.0, x.1))
		});
}
