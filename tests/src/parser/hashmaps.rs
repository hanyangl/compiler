#[cfg(test)]
use sflyn_parser::Parser;

#[cfg(test)]
use super::generate_lexer;

#[cfg(test)]
fn test_hashmap_error(content: &str, expect: usize) {
  let lexer = generate_lexer(content);
  let mut parser = Parser::new(lexer);
  parser.parse_program();

  if parser.errors.len() > 0 {
    parser.show_errors();
    println!("\n");
  }

  assert_eq!(parser.errors.len(), expect);
}

#[test]
fn parser_hashmaps() {
  // HashMap declarations
  test_hashmap_error("let info = { };", 0);
  test_hashmap_error("let info = { 'lang': 'Sflyn' };", 1);
  test_hashmap_error("let info = { lang: 'Sflyn', };", 0);
  test_hashmap_error("let info = { lang: 'Sflyn', year: 2020 };", 0);
  test_hashmap_error("let numbers = { add: (x: number): number => { return x + 2; } };", 0);
  test_hashmap_error("let info = { lang: 'Sflyn', lang: 'Sflyn 2.0' };", 1);
  test_hashmap_error("let info = { lang: { lang: 'Sflyn' } };", 0);

  // HashMap methods.
  let hashmap = "let sflyn = { year: 2020, info: { name: 'Sflyn', author: { name: 'Daniel Solarte', get: (): string => { return 'Author'; } } } };";

  test_hashmap_error(format!("{}\nlet year = sflyn->year1;", hashmap).as_str(), 1);
  test_hashmap_error(format!("{}\nlet year = sflyn->year();", hashmap).as_str(), 1);
  test_hashmap_error(format!("{}\nlet year = sflyn->year->none;", hashmap).as_str(), 1);
  test_hashmap_error(format!("{}\nlet year = sflyn->year;", hashmap).as_str(), 0);
  test_hashmap_error(format!("{}\nlet lang_name = sflyn->info->name1;", hashmap).as_str(), 1);
  test_hashmap_error(format!("{}\nlet lang_name = sflyn->info->name();", hashmap).as_str(), 1);
  test_hashmap_error(format!("{}\nlet lang_name = sflyn->info->name;", hashmap).as_str(), 0);
  test_hashmap_error(format!("{}\nlet author = sflyn->info->author->name1;", hashmap).as_str(), 1);
  test_hashmap_error(format!("{}\nlet author = sflyn->info->author->name();", hashmap).as_str(), 1);
  test_hashmap_error(format!("{}\nlet author = sflyn->info->author->name;", hashmap).as_str(), 0);
  test_hashmap_error(format!("{}\nlet author = sflyn->info->author->get();", hashmap).as_str(), 0);
}
