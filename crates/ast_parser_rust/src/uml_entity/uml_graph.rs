use std::{ops::Index};

use {
    super::uml_fn::UMLFn,
    super::{UMLClass},
    super::{UMLRelation, UMLRelationKind},
};

#[derive(PartialEq, Debug)]
pub struct UMLGraph {
    pub structs: Vec<(String, UMLClass)>,
    pub fns: Vec<UMLFn>,
    relations: Vec<UMLRelation>
}

impl UMLGraph {
    pub fn new() -> UMLGraph {
        UMLGraph { structs: vec![], fns: vec![], relations: vec![]}
    }

    pub fn add_relation(&mut self, rel: UMLRelation) -> () {
        // if relation's from or to not in graph already, it cannot be added
        if (self.get_fn_names().contains(&rel.from) || self.get_struct_names().contains(&rel.from)) &&
                (self.get_fn_names().contains(&rel.to) || self.get_struct_names().contains(&rel.to)) &&
                (&rel.from != &rel.to) {
                    // if new relation's kind is associationUni, then search for associationUni relation with opposite direction and replace it with associationBi
                    if &rel.kind == &UMLRelationKind::UMLAssociationUni {
                        match self.get_relation(&rel.to, &rel.from) {
                            Some(e_rel) => {
                                if &e_rel.kind == &rel.kind {
                                    e_rel.change_relation_kind(UMLRelationKind::UMLAssociationBi);
                                    return
                                }
                            },
                            None => ()
                        }
                    }
                    
                    match self.get_relation(&rel.from, &rel.to) {
                        Some(e_rel) => {
                            // if existing relation's kind has less priority than new relation's, change the relation kind
                            if e_rel.kind < rel.kind {
                                e_rel.change_relation_kind(rel.kind);
                            }
                        },
                        None => {
                            self.relations.push(rel);
                        }
                    }                    
                }
        else {
            dbg!("warning: this graph cannot add Relation now", rel);
        }
        
    }

    pub fn add_struct(&mut self, cls: UMLClass) -> () {
        if self.get_struct_names().contains(&cls.name) {
            self.get_mut_struct(&cls.name).unwrap().merge_from(&mut cls.clone());
        } else {
            let st_name = cls.name.clone();
            self.structs.push((st_name.clone(), cls));
        }
    }

    pub fn add_fn(&mut self, f: UMLFn) -> () {
        self.fns.push(f);
    }

    pub fn get_relations(&self) -> Vec<UMLRelation> {
        let mut relations = self.relations.clone();

        // compare two adjacent relation, if they have same "from" and "to", then the less ordered Relation will not count in
        relations.sort();
        relations.reverse();
        let mut results: Vec<UMLRelation> = vec![];
        for r in relations {
            match results.last() {
                Some(r_other) => if !r.same_objects(r_other) {
                    results.push(r);
                },
                None => { results.push(r) }
            }
        }
        
        self.merge_association(results)
    }

    fn merge_association(&self, relations: Vec<UMLRelation>) -> Vec<UMLRelation> {
        let mut results = vec![];
        // temp vec for storing association relations
        let mut uni_associations: Vec<UMLRelation> = vec![];
        for r in relations {
            match r.kind {
                // compare relation with Uni Association Type with every Relation in uni_associations,
                // if match with opposite relation, push Bi-Association to Results and remove matched relation from uni_associations,
                // if not, push the relation to uni_associations
                UMLRelationKind::UMLAssociationUni => {
                    let mut match_bi_index: Option<usize> = None;
                    for ua_index in 0..uni_associations.len() {
                        if r.opposite_objects(uni_associations.index(ua_index)) {
                            match_bi_index = Some(ua_index);
                            break;
                        }
                    }
                    match match_bi_index {
                        Some(i) => {
                            results.push(UMLRelation::new(&r.from, &r.to, UMLRelationKind::UMLAssociationBi));
                            uni_associations.remove(i);
                        },
                        None => {
                            uni_associations.push(r);
                        }
                    }
                },
                _ => { results.push(r) }
            }
        }

        // finally merge uni_associations to include unmatched association relations
        results.append(&mut uni_associations);
        results
    }

    fn get_mut_struct(&mut self, struct_name: &str) -> Option<&mut UMLClass> {
        match self.structs.iter_mut().find(|(st_name, _)| st_name == struct_name) {
            Some((_, c)) => Some(c),
            None => None
        }
    }

    fn get_struct_names(&self) -> Vec<String> {
        self.structs
            .iter()
            .map(|(st_name, _)| st_name.clone())
            .collect()
    }

    fn get_fn_names(&self) -> Vec<String> {
        self.fns
            .iter()
            .map(|f| f.name.clone())
            .collect()
    }

    fn get_relation(&mut self, from: &str, to: &str) -> Option<&mut UMLRelation> {
        for rel in &mut self.relations {
            if rel.from == from && rel.to == to {
                return Some(rel)
            }
        }
        None
    }
}