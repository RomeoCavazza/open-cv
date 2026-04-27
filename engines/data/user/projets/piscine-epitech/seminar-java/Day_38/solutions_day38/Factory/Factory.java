package Factory;

import java.util.ArrayList;
import java.util.List;

public class Factory {
    
    public Toy create(String toyName) throws NoSuchToyException {
        if ("teddy".equals(toyName)) {
            return new TeddyBear();
        } else if ("gameboy".equals(toyName)) {
            return new Gameboy();
        } else {
            throw new NoSuchToyException("No such toy: " + toyName + ".");
        }
    }
    
    public List<GiftPaper> getPapers(int n) {
        List<GiftPaper> papers = new ArrayList<>();
        for (int i = 0; i < n; i++) {
            papers.add(new GiftPaper());
        }
        return papers;
    }
}
