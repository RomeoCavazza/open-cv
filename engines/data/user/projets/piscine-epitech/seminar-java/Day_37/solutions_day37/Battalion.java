import java.util.List;
import java.util.ArrayList;

public class Battalion {
    private List<Character> characters;

    public Battalion() {
        this.characters = new ArrayList<>();
    }

    public void add(List<? extends Character> newCharacters) {
        characters.addAll(newCharacters);
    }

    public void display() {
        for (Character character : characters) {
            System.out.println(character.getName());
        }
    }

    public boolean fight() {
        if (characters.size() < 2) {
            return false;
        }

        Character first = characters.get(0);
        Character second = characters.get(1);

        int result = first.compareTo(second);

        if (result > 0) {
            // First wins, remove second
            characters.remove(1);
        } else if (result < 0) {
            // Second wins, remove first
            characters.remove(0);
        } else {
            // Tie, remove both
            characters.remove(1);
            characters.remove(0);
        }

        return true;
    }
}

