use tiexiu::cfg::constants::*;
use tiexiu::input::StrCursor;
use tiexiu::state::corectx::CoreCtx;

#[test]
#[ignore = "TODO: Fix ebnf failures"]
fn test_ebnf_parsing() -> tiexiu::Result<()> {
    let ebnf_text = std::fs::read_to_string(TATSU_GRAMMAR_EBNF_PATH)?;
    println!("EBNF grammar length: {} chars", ebnf_text.len());

    let boot = tiexiu::json::boot::boot_grammar()?;
    println!("Boot grammar has {} rules", boot.rules().count());

    let cursor = StrCursor::new(&ebnf_text);
    let mut ctx = CoreCtx::new(cursor);
    ctx.set_trace(true);

    let (_, tree) = boot.parse(ctx)?;
    println!("SUCCESS: Boot grammar parsed EBNF!");
    println!("Tree: {:?}", tree);

    // Basic assertion to verify we got a tree
    assert!(!tree.is_empty(), "Parsed tree should not be empty");

    Ok(())
}
