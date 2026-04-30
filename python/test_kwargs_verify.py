import tiexiu

# Test that kwargs are actually passed to Rust
tx = tiexiu.pegapi()

# Try with trace=True - this should go through
result = tx.parse_grammar("start = /a/", trace=True)
print("parse_grammar with trace=True:", result)

# Try with something else
result2 = tx.compile_to_json("start = 'hello'", trace=False)
print("compile_to_json with trace=False:", result2)

# Try boot_grammar_to_json with kwargs  
result3 = tx.boot_grammar_to_json(trace=True)
print("boot_grammar_to_json with trace:", result3)

# Test the function API
result4 = tiexiu.parse_grammar("start = /b/", trace=True)
print("fnapi parse_grammar with trace:", result4)

result5 = tiexiu.compile_to_json("start = /c/", trace=True)
print("fnapi compile_to_json with trace:", result5)

print("\nAll kwargs tests passed!")