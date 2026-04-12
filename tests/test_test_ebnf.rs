use tiexiu::input::StrCursor;
use tiexiu::state::corectx::CoreCtx;

#[test]
#[ignore]
fn test_ebnf_parsing() {
    let ebnf_text =
        std::fs::read_to_string("grammar/tatsu.tatsu").expect("Failed to read tatsu.tatsu");
    println!("EBNF grammar length: {} chars", ebnf_text.len());

    let boot = tiexiu::json::boot::boot_grammar().expect("Failed to load boot grammar");
    println!("Boot grammar has {} rules", boot.rules().count());

    let cursor = StrCursor::new(&ebnf_text);
    let ctx = CoreCtx::new(cursor);

    match boot.parse(ctx) {
        Ok(s) => {
            println!("SUCCESS: Boot grammar parsed EBNF!");
            println!("Tree: {:?}", s.1);
        }
        Err(failure) => {
            println!("FAILED to parse EBNF: {:?}", failure);
            panic!("Failed to parse EBNF");
        }
    }
}
