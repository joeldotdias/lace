use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lace::lexer::{lexer::Lexer, token::Token};

const LEXABLES: &str = r#"
        let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y
        };

        let result = add(five, ten);
        let greet = "Hi, my age is 10";
        let flag = true;

        if result >= 11 {
            five = five + 1;
        }

        let divide = fn(x, y) {
            let check_zero = fn(a) {
                if a == 0 {
                    flag = false;
                    return;
                }
            };
            check_zero(y);
            x / y
        };
        let quo = divide(result, 0);
"#;

fn provide_lex(n: u64) {
    for _ in 1..n {
        let mut lex = Lexer::new(LEXABLES.into());
        loop {
            let tok = lex.next_token();
            if tok == Token::Eof {
                break;
            }
        }
    }
}

pub fn lex_benchmark(c: &mut Criterion) {
    c.bench_function("Lexer 1 mil tokens", |b| {
        b.iter(|| provide_lex(black_box(10000)))
    });
}

criterion_group!(benches, lex_benchmark);
criterion_main!(benches);
