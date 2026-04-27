package Factory;

import java.util.ArrayList;
import java.util.List;

public class Elf {
    private Toy toy;
    private List<GiftPaper> papers;
    private Factory factory;
    
    public Elf(Factory factory) {
        this.factory = factory;
        this.papers = new ArrayList<>();
    }
    
    public boolean pickToy(String toyName) {
        if (this.toy != null) {
            System.out.println("Minute please?! I'm not that fast.");
            return false;
        }
        
        try {
            this.toy = factory.create(toyName);
            System.out.println("What a nice one! I would have liked to keep it...");
            return true;
        } catch (NoSuchToyException e) {
            System.out.println("I didn't find any " + toyName + ".");
            return false;
        }
    }
    
    public boolean pickPapers(int nb) {
        List<GiftPaper> newPapers = factory.getPapers(nb);
        this.papers.addAll(newPapers);
        return true;
    }
    
    public GiftPaper pack() {
        if (papers == null || papers.isEmpty()) {
            System.out.println("Wait... I can't pack it with my shirt.");
            return null;
        }
        
        if (toy == null) {
            // Retourner un GiftPaper vide (non wrap) et afficher le message
            System.out.println("I don't have any toy, but hey at least it's paper!");
            return papers.remove(0);
        }
        
        GiftPaper paper = papers.remove(0);
        paper.wrap(toy);
        this.toy = null;
        
        System.out.println("And another kid will be happy!");
        return paper;
    }
}
