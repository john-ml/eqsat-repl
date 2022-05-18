use egg::{*};
use std::io::{self, BufRead};

fn main() {
    let toplevel_marker = "toplevel-marker"; // hack to force all queries to be about toplevel expressions
    let rw_sep = "~>";
    let mut rule_number = 0;
    let mut rules: Vec<Rewrite<SymbolLang, ()>> = vec!();
    let mut line = String::new();
    let mut runner = Runner::default();
    loop {
        line.clear();
        io::stdin().lock().read_line(&mut line).expect("Couldn't read line");
        if line == "\n" || line.chars().next() == Some('#') { continue }
        match line.find(rw_sep) {
            // Adding rewrite rules
            Some(i) => {
                let lhs_pat: Pattern<SymbolLang> = (&line[0..i]).parse().unwrap();
                let rhs_pat: Pattern<SymbolLang> = (&line[i+rw_sep.len()..]).parse().unwrap();
                println!("Adding rewrite rule {} {} {}.", &lhs_pat, rw_sep, &rhs_pat);
                let rw: Rewrite<SymbolLang, ()> =
                    Rewrite::new(format!("rule-{}", rule_number), lhs_pat, rhs_pat).unwrap();
                rule_number += 1;
                rules.push(rw);
            }
            None => {
                if line.chars().any(|c| c == '?') {
                    // Queries
                    let pat: Pattern<SymbolLang> = format!("({} {})", toplevel_marker, line).parse().unwrap();
                    println!("Searching for matches...");
                    let matches: Vec<SearchMatches<SymbolLang>> = pat.search(&runner.egraph);
                    let extractor = Extractor::new(&runner.egraph, AstSize);
                    let vars: Vec<Var> = pat.vars();
                    assert!(matches.len() == 1); // there should be only 1 eclass containing the toplevel marker
                    println!("Found match:");
                    let m = &matches[0];
                    for s in &m.substs {
                        for x in &vars {
                            // If we wanted to get ahold EClass to iterate over all possible
                            // instantiations of x, would have to do something like this:
                            // let eclass: &EClass<SymbolLang, ()> = (&runner.egraph)[x's id];
                            let (_, v) = extractor.find_best(*(s.get(*x).unwrap()));
                            println!("  {} ↦ {}", x, v);
                        }
                    }
                } else {
                    // Resetting the e-graph
                    let expr: RecExpr<SymbolLang> = line.parse().unwrap();
                    println!("Saturating e-graph starting from {}...", expr);
                    let expr: RecExpr<SymbolLang> =
                        format!("({} {})", toplevel_marker, line).parse().unwrap(); // hack
                    runner = Runner::default()
                        .with_expr(&expr)
                        // .with_node_limit(1000000)
                        // .with_hook(|runner| {
                        //      println!("Egraph is this big: {}", runner.egraph.total_size());
                        //      Ok(())
                        // })
                        .run(&rules);
                    println!("Saturation stopped. Reason: {:?}", runner.stop_reason);
                }
            }
        }
    }
}
