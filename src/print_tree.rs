use std::path::PathBuf;
use crate::file_tree::FileTree;
use crate::size::Size;

impl FileTree {
    pub fn show(&self, lexico: bool) {
        let racine = self.get_root();
        let taille_racine = self.get_size(racine);
        if let Some(taille) = taille_racine {
            println!("{}    {:?}", taille, racine);
        }

        if lexico == false {

            let mut children_self = self.get_children(&racine); 
            //va trier dans un ordre croissant la taille des children
            children_self.sort_by(|a, b| 
                {
                    let taille_a = self.get_size(a);
                    let taille_b = self.get_size(b);
                    taille_a.cmp(&taille_b)
                });
            for x in &children_self {
                let taille_children = self.get_size(x);
                if let Some(taille) = taille_children {
                    println!("      {}    {:?}", taille, x);
                }
            }

        } else {
            let mut children_self = self.get_children(&racine); 
            //va trier dans un ordre croissant lexicographique les path des children
            children_self.sort_by(|a, b| 
                {
                    a.cmp(&b)
                });
            for x in &children_self {
                let taille_children = self.get_size(x);
                if let Some(taille) = taille_children {
                    println!("      {}    {:?}", taille, x);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_show_tri_taille() {
        let chemin = std::path::Path::new("src/test/sous_dossier");
        let tree = FileTree::new(chemin);
        if let Ok(tree) = tree {
            tree.show(false);
        }
    }

    #[test]
    fn test_show_tri_lexico() {
        let chemin = std::path::Path::new("src/test/sous_dossier");
        let tree = FileTree::new(chemin);
        if let Ok(tree) = tree {
            tree.show(true);
        }
    }
}
