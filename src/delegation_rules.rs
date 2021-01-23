// use rify::Entity::{Bound, Unbound};

// After considering several options, I chose option 3.
// Option 1:
// if   a? rdf:type dock:SubjectUnbound
// and  b? rdf:type dock:Mentioned
// then a? dock:allowsSubject b?
//
// if   a? rdf:type dock:PredicateUnbound
// and  b? rdf:type dock:Mentioned
// then a? dock:allowsPredicate b?
//
// if   a? rdf:type dock:ObjectUnbound
// and  b? rdf:type dock:Mentioned
// then a? dock:allowsObject b?
//
// if   a? b? c?
// then a? rdf:type dock:Mentioned
// and  b? rdf:type dock:Mentioned
// and  c? rdf:type dock:Mentioned

// Option 2:
// if   a?   dock:claims           [s? p? o?]
// and  pol? { rdf:type dock:SubjectUnbound   } OR { dock:allowsSubject   s? }
// and  pol? { rdf:type dock:PredicateUnbound } OR { dock:allowsPredicate p? }
// and  pol? { rdf:type dock:ObjectUnbound    } OR { dock:allowsObject    o? }
// and  a?   dock:mayClaim         pol?
// then s? p? o?

// Option 3
// if   ?a dock:claims [rdf:subject ?s ; rdf:predicate ?p ; rdf:object ?o]
// and  ?a dock:mayClaim [
//     dock:allowSubjects   { [ rdfs:member ?s ] } OR { dock:Anything } ;
//     dock:allowPredicates { [ rdfs:member ?p ] } OR { dock:Anything } ;
//     dock:allowObjects    { [ rdfs:member ?o ] } OR { dock:Anything } ;
// ]
// then ?s ?p ?o

// Option 4:
// Maybe this is not the best course. Predicates with domain and range often entail the types of
// their subjects/objects. This may interact in unexpected ways with some policies.
// if   ?a dock:claims [rdf:subject ?s ; rdf:predicate ?p ; rdf:object ?o]
// and  ?a dock:mayClaim [
//     dock:allowSubjects   ?st ;
//     dock:allowPredicates ?pt ;
//     dock:allowObjects    ?ot ;
// ]
// and  ?s rdf:type ?st
// and  ?p rdf:type ?pt
// and  ?o rdf:type ?ot
// then ?s ?p ?o

// pub fn rules() -> Vec<[Vec<[rify::Entity<&'static str, RdfNode>; 3]>; 2]> {
//     vec![
//         [
//             vec![
//                 [
//                     Unbound("a"),
//                     Bound(Iri("https://dock.io/rdf/alpha/claims".to_string())),
//                     Unbound("c"),
//                 ],
//                 [
//                     Unbound("c"),
//                     Bound(Iri(
//                         "http://www.w3.org/1999/02/22-rdf-syntax-ns#subject".to_string()
//                     )),
//                     Unbound("s"),
//                 ],
//                 [
//                     Unbound("c"),
//                     Bound(Iri(
//                         "http://www.w3.org/1999/02/22-rdf-syntax-ns#predicate".to_string()
//                     )),
//                     Unbound("p"),
//                 ],
//                 [
//                     Unbound("c"),
//                     Bound(Iri(
//                         "http://www.w3.org/1999/02/22-rdf-syntax-ns#object".to_string()
//                     )),
//                     Unbound("o"),
//                 ],
//                 [
//                     Unbound("a"),
//                     Bound(Iri("https://dock.io/rdf/alpha/mayClaim".to_string())),
//                     Unbound("pol"),
//                 ],
//                 [
//                     Unbound("pol"),
//                     Bound(Iri("https://dock.io/rdf/alpha/allowSubjects".to_string())),
//                     Unbound("subs"),
//                 ],
//                 [
//                     Unbound("subs"),
//                     Bound(Iri(
//                         "http://www.w3.org/2000/01/rdf-schema#member".to_string()
//                     )),
//                     Unbound("s"),
//                 ],
//                 [
//                     Unbound("pol"),
//                     Bound(Iri("https://dock.io/rdf/alpha/allowPredicates".to_string())),
//                     Unbound("preds"),
//                 ],
//                 [
//                     Unbound("preds"),
//                     Bound(Iri(
//                         "http://www.w3.org/2000/01/rdf-schema#member".to_string()
//                     )),
//                     Unbound("p"),
//                 ],
//                 [
//                     Unbound("pol"),
//                     Bound(Iri("https://dock.io/rdf/alpha/allowObjects".to_string())),
//                     Unbound("obs"),
//                 ],
//                 [
//                     Unbound("obs"),
//                     Bound(Iri(
//                         "http://www.w3.org/2000/01/rdf-schema#member".to_string()
//                     )),
//                     Unbound("o"),
//                 ],
//             ],
//             vec![[Unbound("s"), Unbound("p"), Unbound("o")]],
//         ],
//         [
//             vec![
//                 [
//                     Unbound("a"),
//                     Bound(Iri("https://dock.io/rdf/alpha/claims".to_string())),
//                     Unbound("c"),
//                 ],
//                 [
//                     Unbound("c"),
//                     Bound(Iri(
//                         "http://www.w3.org/1999/02/22-rdf-syntax-ns#subject".to_string()
//                     )),
//                     Unbound("s"),
//                 ],
//                 [
//                     Unbound("c"),
//                     Bound(Iri(
//                         "http://www.w3.org/1999/02/22-rdf-syntax-ns#predicate".to_string()
//                     )),
//                     Unbound("p"),
//                 ],
//                 [
//                     Unbound("c"),
//                     Bound(Iri(
//                         "http://www.w3.org/1999/02/22-rdf-syntax-ns#object".to_string()
//                     )),
//                     Unbound("o"),
//                 ],
//                 [
//                     Unbound("a"),
//                     Bound(Iri("https://dock.io/rdf/alpha/mayClaim".to_string())),
//                     Unbound("pol"),
//                 ],
//                 [
//                     Unbound("pol"),
//                     Bound(Iri("https://dock.io/rdf/alpha/allowSubjects".to_string())),
//                     Unbound("subs"),
//                 ],
//                 [
//                     Unbound("subs"),
//                     Bound(Iri(
//                         "http://www.w3.org/2000/01/rdf-schema#member".to_string()
//                     )),
//                     Unbound("s"),
//                 ],
//                 [
//                     Unbound("pol"),
//                     Bound(Iri("https://dock.io/rdf/alpha/allowPredicates".to_string())),
//                     Unbound("preds"),
//                 ],
//                 [
//                     Unbound("preds"),
//                     Bound(Iri(
//                         "http://www.w3.org/2000/01/rdf-schema#member".to_string()
//                     )),
//                     Unbound("p"),
//                 ],
//                 [
//                     Unbound("pol"),
//                     Bound(Iri("https://dock.io/rdf/alpha/allowObjects".to_string())),
//                     Bound(Iri("https://dock.io/rdf/alpha/ANYTHING".to_string())),
//                 ],
//             ],
//             vec![[Unbound("s"), Unbound("p"), Unbound("o")]],
//         ],
//         [
//             vec![
//                 [
//                     Unbound("a"),
//                     Bound(Iri("https://dock.io/rdf/alpha/claims".to_string())),
//                     Unbound("c"),
//                 ],
//                 [
//                     Unbound("c"),
//                     Bound(Iri(
//                         "http://www.w3.org/1999/02/22-rdf-syntax-ns#subject".to_string()
//                     )),
//                     Unbound("s"),
//                 ],
//                 [
//                     Unbound("c"),
//                     Bound(Iri(
//                         "http://www.w3.org/1999/02/22-rdf-syntax-ns#predicate".to_string()
//                     )),
//                     Unbound("p"),
//                 ],
//                 [
//                     Unbound("c"),
//                     Bound(Iri(
//                         "http://www.w3.org/1999/02/22-rdf-syntax-ns#object".to_string()
//                     )),
//                     Unbound("o"),
//                 ],
//                 [
//                     Unbound("a"),
//                     Bound(Iri("https://dock.io/rdf/alpha/mayClaim".to_string())),
//                     Unbound("pol"),
//                 ],
//                 [
//                     Unbound("pol"),
//                     Bound(Iri("https://dock.io/rdf/alpha/allowSubjects".to_string())),
//                     Unbound("subs"),
//                 ],
//                 [
//                     Unbound("subs"),
//                     Bound(Iri(
//                         "http://www.w3.org/2000/01/rdf-schema#member".to_string()
//                     )),
//                     Unbound("s"),
//                 ],
//                 [
//                     Unbound("pol"),
//                     Bound(Iri("https://dock.io/rdf/alpha/allowPredicates".to_string())),
//                     Bound(Iri("https://dock.io/rdf/alpha/ANYTHING".to_string())),
//                 ],
//                 [
//                     Unbound("pol"),
//                     Bound(Iri("https://dock.io/rdf/alpha/allowObjects".to_string())),
//                     Unbound("obs"),
//                 ],
//                 [
//                     Unbound("obs"),
//                     Bound(Iri(
//                         "http://www.w3.org/2000/01/rdf-schema#member".to_string()
//                     )),
//                     Unbound("o"),
//                 ],
//             ],
//             vec![[Unbound("s"), Unbound("p"), Unbound("o")]],
//         ],
//         [
//             vec![
//                 [
//                     Unbound("a"),
//                     Bound(Iri("https://dock.io/rdf/alpha/claims".to_string())),
//                     Unbound("c"),
//                 ],
//                 [
//                     Unbound("c"),
//                     Bound(Iri(
//                         "http://www.w3.org/1999/02/22-rdf-syntax-ns#subject".to_string()
//                     )),
//                     Unbound("s"),
//                 ],
//                 [
//                     Unbound("c"),
//                     Bound(Iri(
//                         "http://www.w3.org/1999/02/22-rdf-syntax-ns#predicate".to_string()
//                     )),
//                     Unbound("p"),
//                 ],
//                 [
//                     Unbound("c"),
//                     Bound(Iri(
//                         "http://www.w3.org/1999/02/22-rdf-syntax-ns#object".to_string()
//                     )),
//                     Unbound("o"),
//                 ],
//                 [
//                     Unbound("a"),
//                     Bound(Iri("https://dock.io/rdf/alpha/mayClaim".to_string())),
//                     Unbound("pol"),
//                 ],
//                 [
//                     Unbound("pol"),
//                     Bound(Iri("https://dock.io/rdf/alpha/allowSubjects".to_string())),
//                     Unbound("subs"),
//                 ],
//                 [
//                     Unbound("subs"),
//                     Bound(Iri(
//                         "http://www.w3.org/2000/01/rdf-schema#member".to_string()
//                     )),
//                     Unbound("s"),
//                 ],
//                 [
//                     Unbound("pol"),
//                     Bound(Iri("https://dock.io/rdf/alpha/allowPredicates".to_string())),
//                     Bound(Iri("https://dock.io/rdf/alpha/ANYTHING".to_string())),
//                 ],
//                 [
//                     Unbound("pol"),
//                     Bound(Iri("https://dock.io/rdf/alpha/allowObjects".to_string())),
//                     Bound(Iri("https://dock.io/rdf/alpha/ANYTHING".to_string())),
//                 ],
//             ],
//             vec![[Unbound("s"), Unbound("p"), Unbound("o")]],
//         ],
//         [
//             vec![
//                 [
//                     Unbound("a"),
//                     Bound(Iri("https://dock.io/rdf/alpha/claims".to_string())),
//                     Unbound("c"),
//                 ],
//                 [
//                     Unbound("c"),
//                     Bound(Iri(
//                         "http://www.w3.org/1999/02/22-rdf-syntax-ns#subject".to_string()
//                     )),
//                     Unbound("s"),
//                 ],
//                 [
//                     Unbound("c"),
//                     Bound(Iri(
//                         "http://www.w3.org/1999/02/22-rdf-syntax-ns#predicate".to_string()
//                     )),
//                     Unbound("p"),
//                 ],
//                 [
//                     Unbound("c"),
//                     Bound(Iri(
//                         "http://www.w3.org/1999/02/22-rdf-syntax-ns#object".to_string()
//                     )),
//                     Unbound("o"),
//                 ],
//                 [
//                     Unbound("a"),
//                     Bound(Iri("https://dock.io/rdf/alpha/mayClaim".to_string())),
//                     Unbound("pol"),
//                 ],
//                 [
//                     Unbound("pol"),
//                     Bound(Iri("https://dock.io/rdf/alpha/allowSubjects".to_string())),
//                     Bound(Iri("https://dock.io/rdf/alpha/ANYTHING".to_string())),
//                 ],
//                 [
//                     Unbound("pol"),
//                     Bound(Iri("https://dock.io/rdf/alpha/allowPredicates".to_string())),
//                     Unbound("preds"),
//                 ],
//                 [
//                     Unbound("preds"),
//                     Bound(Iri(
//                         "http://www.w3.org/2000/01/rdf-schema#member".to_string()
//                     )),
//                     Unbound("p"),
//                 ],
//                 [
//                     Unbound("pol"),
//                     Bound(Iri("https://dock.io/rdf/alpha/allowObjects".to_string())),
//                     Unbound("obs"),
//                 ],
//                 [
//                     Unbound("obs"),
//                     Bound(Iri(
//                         "http://www.w3.org/2000/01/rdf-schema#member".to_string()
//                     )),
//                     Unbound("o"),
//                 ],
//             ],
//             vec![[Unbound("s"), Unbound("p"), Unbound("o")]],
//         ],
//         [
//             vec![
//                 [
//                     Unbound("a"),
//                     Bound(Iri("https://dock.io/rdf/alpha/claims".to_string())),
//                     Unbound("c"),
//                 ],
//                 [
//                     Unbound("c"),
//                     Bound(Iri(
//                         "http://www.w3.org/1999/02/22-rdf-syntax-ns#subject".to_string()
//                     )),
//                     Unbound("s"),
//                 ],
//                 [
//                     Unbound("c"),
//                     Bound(Iri(
//                         "http://www.w3.org/1999/02/22-rdf-syntax-ns#predicate".to_string()
//                     )),
//                     Unbound("p"),
//                 ],
//                 [
//                     Unbound("c"),
//                     Bound(Iri(
//                         "http://www.w3.org/1999/02/22-rdf-syntax-ns#object".to_string()
//                     )),
//                     Unbound("o"),
//                 ],
//                 [
//                     Unbound("a"),
//                     Bound(Iri("https://dock.io/rdf/alpha/mayClaim".to_string())),
//                     Unbound("pol"),
//                 ],
//                 [
//                     Unbound("pol"),
//                     Bound(Iri("https://dock.io/rdf/alpha/allowSubjects".to_string())),
//                     Bound(Iri("https://dock.io/rdf/alpha/ANYTHING".to_string())),
//                 ],
//                 [
//                     Unbound("pol"),
//                     Bound(Iri("https://dock.io/rdf/alpha/allowPredicates".to_string())),
//                     Unbound("preds"),
//                 ],
//                 [
//                     Unbound("preds"),
//                     Bound(Iri(
//                         "http://www.w3.org/2000/01/rdf-schema#member".to_string()
//                     )),
//                     Unbound("p"),
//                 ],
//                 [
//                     Unbound("pol"),
//                     Bound(Iri("https://dock.io/rdf/alpha/allowObjects".to_string())),
//                     Bound(Iri("https://dock.io/rdf/alpha/ANYTHING".to_string())),
//                 ],
//             ],
//             vec![[Unbound("s"), Unbound("p"), Unbound("o")]],
//         ],
//         [
//             vec![
//                 [
//                     Unbound("a"),
//                     Bound(Iri("https://dock.io/rdf/alpha/claims".to_string())),
//                     Unbound("c"),
//                 ],
//                 [
//                     Unbound("c"),
//                     Bound(Iri(
//                         "http://www.w3.org/1999/02/22-rdf-syntax-ns#subject".to_string()
//                     )),
//                     Unbound("s"),
//                 ],
//                 [
//                     Unbound("c"),
//                     Bound(Iri(
//                         "http://www.w3.org/1999/02/22-rdf-syntax-ns#predicate".to_string()
//                     )),
//                     Unbound("p"),
//                 ],
//                 [
//                     Unbound("c"),
//                     Bound(Iri(
//                         "http://www.w3.org/1999/02/22-rdf-syntax-ns#object".to_string()
//                     )),
//                     Unbound("o"),
//                 ],
//                 [
//                     Unbound("a"),
//                     Bound(Iri("https://dock.io/rdf/alpha/mayClaim".to_string())),
//                     Unbound("pol"),
//                 ],
//                 [
//                     Unbound("pol"),
//                     Bound(Iri("https://dock.io/rdf/alpha/allowSubjects".to_string())),
//                     Bound(Iri("https://dock.io/rdf/alpha/ANYTHING".to_string())),
//                 ],
//                 [
//                     Unbound("pol"),
//                     Bound(Iri("https://dock.io/rdf/alpha/allowPredicates".to_string())),
//                     Bound(Iri("https://dock.io/rdf/alpha/ANYTHING".to_string())),
//                 ],
//                 [
//                     Unbound("pol"),
//                     Bound(Iri("https://dock.io/rdf/alpha/allowObjects".to_string())),
//                     Unbound("obs"),
//                 ],
//                 [
//                     Unbound("obs"),
//                     Bound(Iri(
//                         "http://www.w3.org/2000/01/rdf-schema#member".to_string()
//                     )),
//                     Unbound("o"),
//                 ],
//             ],
//             vec![[Unbound("s"), Unbound("p"), Unbound("o")]],
//         ],
//         [
//             vec![
//                 [
//                     Unbound("a"),
//                     Bound(Iri("https://dock.io/rdf/alpha/claims".to_string())),
//                     Unbound("c"),
//                 ],
//                 [
//                     Unbound("c"),
//                     Bound(Iri(
//                         "http://www.w3.org/1999/02/22-rdf-syntax-ns#subject".to_string()
//                     )),
//                     Unbound("s"),
//                 ],
//                 [
//                     Unbound("c"),
//                     Bound(Iri(
//                         "http://www.w3.org/1999/02/22-rdf-syntax-ns#predicate".to_string()
//                     )),
//                     Unbound("p"),
//                 ],
//                 [
//                     Unbound("c"),
//                     Bound(Iri(
//                         "http://www.w3.org/1999/02/22-rdf-syntax-ns#object".to_string()
//                     )),
//                     Unbound("o"),
//                 ],
//                 [
//                     Unbound("a"),
//                     Bound(Iri("https://dock.io/rdf/alpha/mayClaim".to_string())),
//                     Unbound("pol"),
//                 ],
//                 [
//                     Unbound("pol"),
//                     Bound(Iri("https://dock.io/rdf/alpha/allowSubjects".to_string())),
//                     Bound(Iri("https://dock.io/rdf/alpha/ANYTHING".to_string())),
//                 ],
//                 [
//                     Unbound("pol"),
//                     Bound(Iri("https://dock.io/rdf/alpha/allowPredicates".to_string())),
//                     Bound(Iri("https://dock.io/rdf/alpha/ANYTHING".to_string())),
//                 ],
//                 [
//                     Unbound("pol"),
//                     Bound(Iri("https://dock.io/rdf/alpha/allowObjects".to_string())),
//                     Bound(Iri("https://dock.io/rdf/alpha/ANYTHING".to_string())),
//                 ],
//             ],
//             vec![[Unbound("s"), Unbound("p"), Unbound("o")]],
//         ],
//     ]
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::util::prefix;

//     pub fn dock(suffix: &str) -> rify::Entity<&str, RdfNode> {
//         Bound(prefix::dock(suffix))
//     }

//     pub fn rdf(suffix: &str) -> rify::Entity<&str, RdfNode> {
//         Bound(prefix::rdf(suffix))
//     }

//     pub fn rdfs(suffix: &str) -> rify::Entity<&str, RdfNode> {
//         Bound(prefix::rdfs(suffix))
//     }

//     fn gen_delegation_rules() -> Vec<[Vec<[rify::Entity<&'static str, RdfNode>; 3]>; 2]> {
//         let ifs = prod(&[
//             &[vec![
//                 [Unbound("a"), dock("claims"), Unbound("c")],
//                 [Unbound("c"), rdf("subject"), Unbound("s")],
//                 [Unbound("c"), rdf("predicate"), Unbound("p")],
//                 [Unbound("c"), rdf("object"), Unbound("o")],
//                 [Unbound("a"), dock("mayClaim"), Unbound("pol")],
//             ]],
//             &[
//                 vec![
//                     [Unbound("pol"), dock("allowSubjects"), Unbound("subs")],
//                     [Unbound("subs"), rdfs("member"), Unbound("s")],
//                 ],
//                 vec![[Unbound("pol"), dock("allowSubjects"), dock("ANYTHING")]],
//             ],
//             &[
//                 vec![
//                     [Unbound("pol"), dock("allowPredicates"), Unbound("preds")],
//                     [Unbound("preds"), rdfs("member"), Unbound("p")],
//                 ],
//                 vec![[Unbound("pol"), dock("allowPredicates"), dock("ANYTHING")]],
//             ],
//             &[
//                 vec![
//                     [Unbound("pol"), dock("allowObjects"), Unbound("obs")],
//                     [Unbound("obs"), rdfs("member"), Unbound("o")],
//                 ],
//                 vec![[Unbound("pol"), dock("allowObjects"), dock("ANYTHING")]],
//             ],
//         ]);
//         ifs.iter()
//             .map(|if_all| {
//                 [
//                     cat(if_all.iter().cloned()),
//                     vec![[Unbound("s"), Unbound("p"), Unbound("o")]],
//                 ]
//             })
//             .collect()
//     }

//     fn cat<T, L: IntoIterator<Item = T>, Ls: IntoIterator<Item = L>>(ls: Ls) -> Vec<T> {
//         ls.into_iter().flatten().collect()
//     }

//     // n-dimensional cartesian product
//     fn prod<T: Clone>(inputs: &[&[T]]) -> Vec<Vec<T>> {
//         fn p<T: Clone>(inputs: &[&[T]], stack: &mut Vec<T>, ret: &mut Vec<Vec<T>>) {
//             match inputs.split_first() {
//                 None => ret.push(stack.clone()),
//                 Some((first, rest)) => {
//                     for t in first.iter() {
//                         stack.push(t.clone());
//                         p(rest, stack, ret);
//                         stack.pop();
//                     }
//                 }
//             }
//         }

//         let mut ret = Vec::with_capacity(inputs.iter().map(|a| a.len()).fold(1, |a, b| a * b));
//         let mut stack = Vec::with_capacity(inputs.len());
//         p(inputs, &mut stack, &mut ret);
//         ret
//     }

//     #[test]
//     fn tprod() {
//         assert_eq!(prod(&[&[], &['a', 'b']]).len(), 0);
//         assert_eq!(&prod(&[&['a', 'b']]), &[&['a'], &['b']]);
//         assert_eq!(
//             &prod(&[&['a', 'b'], &['c']]),
//             &[vec!['a', 'c'], vec!['b', 'c']]
//         );
//         assert_eq!(
//             &prod(&[&['a', 'b'], &['c', 'd']]),
//             &[
//                 vec!['a', 'c'],
//                 vec!['a', 'd'],
//                 vec!['b', 'c'],
//                 vec!['b', 'd']
//             ]
//         );
//         assert_eq!(
//             &prod(&[&['1'], &['a', 'b'], &['2'], &['c', 'd'], &['3']]),
//             &[
//                 vec!['1', 'a', '2', 'c', '3'],
//                 vec!['1', 'a', '2', 'd', '3'],
//                 vec!['1', 'b', '2', 'c', '3'],
//                 vec!['1', 'b', '2', 'd', '3']
//             ]
//         );
//     }

//     #[test]
//     fn delegation_rules_approval_test() {
//         assert_eq!(rules(), gen_delegation_rules());
//     }
// }
