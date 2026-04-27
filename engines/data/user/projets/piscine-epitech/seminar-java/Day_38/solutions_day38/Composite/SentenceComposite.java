package Composite;

import java.util.ArrayList;
import java.util.List;

public class SentenceComposite implements Sentence {
    private List<Sentence> childSentence;
    
    public SentenceComposite() {
        this.childSentence = new ArrayList<>();
    }
    
    @Override
    public void print() {
        for (Sentence sentence : childSentence) {
            sentence.print();
        }
    }
    
    public void add(Sentence sentence) {
        childSentence.add(sentence);
    }
    
    public void remove(Sentence sentence) {
        childSentence.remove(sentence);
    }
}
