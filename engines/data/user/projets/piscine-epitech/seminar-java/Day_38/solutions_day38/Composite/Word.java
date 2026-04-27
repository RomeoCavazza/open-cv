package Composite;

public class Word implements Sentence {
    private String word;
    
    public Word(String word) {
        this.word = word;
    }
    
    @Override
    public void print() {
        System.out.println(word);
    }
}
